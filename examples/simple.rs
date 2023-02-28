use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use hazelcast_rs::client::HazelcastClient;
use hazelcast_rs::config::ClientConfig;

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
    let mut client = HazelcastClient::new(client_config).await;
    client.start().await;
    //
    // let mut state = SomeState {
    //     names: NestedStateContext::new_root(WeakRegistryProxy::new(
    //         client.get_map("users").await.to_weak_map(),
    //     )),
    // };
    //
    // let user: ObservableArc<_> = User {
    //     name: "something".to_string(),
    // }
    // .into();
    // state.names.add(user.clone()).await;

    /*



    println!("{:#?}", state.names.get_owned("something").await);
    println!("{:#?}", state.names.get("something").await);
    drop(user);
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("{:#?}", state.names.get_owned("something").await);
    println!("{:#?}", state.names.get("something").await);

     */
}
