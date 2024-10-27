use ecoblock_tangle_core::network::{Network, peer::Peer};
use tokio;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut network = Network::new();
    
    let peer1 = Peer::new("peer1", "127.0.0.1:8081");
    let peer2 = Peer::new("peer2", "127.0.0.1:8082");

    network.add_peer(peer1.clone()).await;
    network.add_peer(peer2.clone()).await;

    println!("Network initialized with peers: {:?}", network.peers);

    let communication = Arc::clone(&network.communication);

    tokio::spawn(async move {
        let comm = communication.lock().await;
        comm.start_server("127.0.0.1:8081").await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    network.communication.lock().await
        .send_message("127.0.0.1:8081", "Hello from peer2!")
        .await
        .unwrap();
}