use tokio::sync::Mutex;  
use std::sync::Arc;
use communication::Communication;

pub mod peer;
pub mod message;
pub mod communication;

pub struct Network {
    pub peers: Vec<peer::Peer>,
    pub communication: Arc<Mutex<Communication>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            peers: Vec::new(),
            communication: Arc::new(Mutex::new(Communication::new())),        }
    }

    //Method to add a peer to the network
    pub async fn add_peer(&mut self, peer: peer::Peer) {
        self.peers.push(peer.clone());
        let mut comm = self.communication.lock().await;
        comm.connect_peer(peer);
        }

    //Method to remove a peer from the network
    pub async fn remove_peer(&mut self, peer_id: &str) {
        self.peers.retain(|peer| peer.id != peer_id);
        let mut comm = self.communication.lock().await;
        comm.disconnect_peer(peer_id);
    }

    //Method to broadcast a message to all connected peers
    pub async fn broadcast_message(&self, sender_id: &str, content: &str) {
        let comm = self.communication.lock().await;
        comm.broadcast_message(sender_id, content);
    }
}