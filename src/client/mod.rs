use crate::cluster::failover::ClusterFailoverService;
use crate::cluster::service::ClusterService;
use crate::config::ClientConfig;
use crate::connection::manager::ConnectionManager;
use crate::connection::registry::ConnectionRegistry;
use crate::invocation::service::InvocationService;
use crate::lifecycle_service::LifecycleService;
use crate::listener::service::ListenerService;
use crate::partition_service::PartitionService;
use crate::proxy::manager::ProxyManager;
use crate::proxy::map_proxy::MapProxy;
use crate::proxy::multimap_proxy::MultiMapProxy;
use crate::serialization::schema_service::SchemaService;
use crate::serialization::serializable::Serializable;
use crate::serialization::service::SerializationServiceV1;
use async_actor::inject::assisted_inject::AssistedInstantiable;
use async_actor::inject::injectable_instance::ManuallyInjectableInstance;
use async_actor::inject::InjectorHandle;
use async_actor::system::HasHandleWrapper;
use async_actor_proc::{actor, assisted_factory, Component, Injectable};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Component)]
pub struct HazelcastClient {
    connection_manager: Arc<ConnectionManager>,
    active: bool,
    cluster_failover_service: Arc<ClusterFailoverService>,
    cluster_service: Arc<ClusterService>,
    connection_registry: Arc<ConnectionRegistry>,
    partition_service: Arc<PartitionService>,
    serialization_service: Arc<SerializationServiceV1>,
    schema_service: Arc<SchemaService>,
    proxy_manager: Arc<ProxyManager>,
    lifecycle_service: Arc<LifecycleService>,
}

#[actor]
impl HazelcastClient {
    async fn init(&self) {
        self.lifecycle_service.start().await;
    }

    pub async fn start(&mut self) {
        if self.active {
            return;
        }
        self.active = true;
        //todo: Add heartbeat manager start
        self.connection_manager.connect_to_cluster().await;
    }

    pub async fn get_map<K, V>(&self, name: String) -> MapProxy<K, V>
    where
        K: Clone + Send + Sync + Serializable + 'static,
        V: Clone + Send + Sync + Serializable + 'static,
    {
        self.proxy_manager.get_or_create_proxy(name, true).await
    }

    pub async fn get_multimap<K, V>(&self, name: String) -> MultiMapProxy<K, V>
    where
        K: Clone + Send + Sync + Serializable + 'static,
        V: Clone + Send + Sync + Serializable + 'static,
    {
        self.proxy_manager.get_or_create_proxy(name, true).await
    }
}

#[async_trait::async_trait]
impl AssistedInstantiable<HazelcastClientInstantiationData> for HazelcastClient {
    async fn instantiate(
        injector: InjectorHandle,
        params: HazelcastClientInstantiationData,
    ) -> Self {
        let config = params.config;

        let cluster_failover_service = Arc::new(ClusterFailoverService::new(vec![config.clone()]));
        let cluster_service = Arc::new(ClusterService::new(
            config.clone(),
            cluster_failover_service.clone(),
        ));
        let connection_strategy = config.connection_strategy.read().await;
        let network = config.network.read().await;

        let connection_registry = Arc::new(ConnectionRegistry::new(
            connection_strategy.async_start,
            connection_strategy.reconnect_mode.clone(),
            network.smart_routing,
            cluster_service.clone(),
        ));
        let invocation_service = Arc::new(InvocationService::new(config.clone()));
        let schema_service = Arc::new(SchemaService::new(connection_registry.clone()));
        let serialization_service = Arc::new(SerializationServiceV1::new(
            config.serialization.read().await.clone(),
            schema_service.clone(),
        ));
        let partition_service = Arc::new(PartitionService::new(serialization_service.clone()));
        let lifecycle_service = Arc::new(LifecycleService::new());

        let connection_manager = Arc::new(
            ConnectionManager::new(
                config.clone(),
                cluster_failover_service.clone(),
                cluster_service.clone(),
                connection_registry.clone(),
                invocation_service.clone(),
                partition_service.clone(),
                lifecycle_service.clone(),
            )
            .await,
        );

        let listener_service = Arc::new(ListenerService::new(
            invocation_service.clone(),
            connection_manager.clone(),
        ));
        let proxy_manager = Arc::new(ProxyManager::new(
            partition_service.clone(),
            connection_registry.clone(),
            invocation_service.clone(),
            serialization_service.clone(),
            listener_service.clone(),
            cluster_service.clone(),
        ));

        let client = HazelcastClient {
            proxy_manager,
            connection_manager,
            active: false,
            cluster_service: cluster_service.clone(),
            cluster_failover_service,
            connection_registry,
            partition_service,
            schema_service,
            serialization_service,
            lifecycle_service,
        };
        client.init().await;
        client
    }
}

pub struct HazelcastClientInstantiationData {
    config: Arc<ClientConfig>,
}

impl HazelcastClientInstantiationData {
    pub fn new(config: Arc<ClientConfig>) -> Self {
        Self { config }
    }
}

impl ManuallyInjectableInstance for HazelcastClientHandle {
    type Inner = HazelcastClient;
}

#[assisted_factory]
pub trait HazelcastClientFactory {
    async fn create(&self, config: Arc<ClientConfig>) -> HazelcastClient;
}
