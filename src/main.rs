use ecoblock_tangle_core::network::{Network, peer::Peer};

fn main() {
    let mut network = Network::new();
    
    let peer1 = Peer::new("peer1", "192.168.1.1:8080");
    let peer2 = Peer::new("peer2", "192.168.1.2:8080");

    network.add_peer(peer1);
    network.add_peer(peer2);
    
    println!("Network initialized with peers: {:?}", network.peers);
}