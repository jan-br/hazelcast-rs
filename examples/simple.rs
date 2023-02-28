#![feature(fn_traits)]

use async_actor::inject::Injector;
use async_actor::system::Component;
use async_actor_proc::{actor, assisted_factory, AssistedInstantiable, Component};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use hazelcast_rs::client::{
    HazelcastClient, HazelcastClientFactory, HazelcastClientFactoryImpl,
    HazelcastClientFactoryImplHandle, HazelcastClientHandle,
};
use hazelcast_rs::config::ClientConfig;
use hazelcast_rs::proxy::weak_registry_proxy::{RegistryEntry, WeakRegistryProxy};
use hazelcast_rs::serialization::serializable::Serializable;
use hazelcast_rs::serialization::serializer::Serializer;
use hazelcast_rs::serialization::service::SerializationServiceV1;

#[tokio::main]
async fn main() {
    let client_config = Arc::new(
        ClientConfig::default()
            .cluster_name("hello-world".to_string())
            .network(|mut network| {
                network
                    .cluster_members
                    .push(SocketAddr::V4(SocketAddrV4::new(
                        Ipv4Addr::new(127, 0, 0, 1),
                        5701,
                    )));
            })
            .await,
    );
    let injector = Injector::default();
    let hazelcast_client_factory = injector.get::<HazelcastClientFactoryImpl>().await;
    let client =
        async_actor::system::Component::start(hazelcast_client_factory.create(client_config).await);
    client.start().await;
    injector.bind_value::<HazelcastClient>(client).await;
    let client = injector.get::<HazelcastClient>().await;
    let some_state_factory = injector.get::<SomeStateFactoryImpl>().await;
    let some_state = some_state_factory.create().await;
    some_state.start();
}

#[derive(Clone)]
struct User {
    name: String,
}

impl Serializable for User {
    fn get_serializer(&self, service: &SerializationServiceV1) -> Arc<dyn Serializer<Box<Self>>> {
        todo!()
    }
}

impl RegistryEntry for User {
    type Key = String;

    fn get_key(&self) -> Self::Key {
        self.name.clone()
    }
}

#[derive(Component, AssistedInstantiable)]
pub struct SomeState {
    #[inject]
    hazelcast_client: HazelcastClientHandle,
    #[inject_default]
    users: WeakRegistryProxy<User>,
}

#[actor]
impl SomeState {}

#[assisted_factory]
pub trait SomeStateFactory {
    async fn create(&self) -> SomeState;
}
