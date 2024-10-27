use super::communication::Communication;

pub mod peer;
pub mod message;
pub mod communication;

pub struct Network {
    pub peers: Vec<peer::Peer>,
    communication: Communication
}

impl Network {
    pub fn new() -> Self {
        Network {
            peers: Vec::new(),
            communication: Communication::new()
        }
    }

    //Method to add a peer to the network
    pub fn add_peer(&mut self, peer: peer::Peer) {
        self.peers.push(peer.clone());
        self.communication.connect_peer(peer);
    }

    //Method to remove a peer from the network
    pub fn remove_peer(&mut self, peer_id : &str){
        self.peers.retain(|p| p.id != peer_id);
        self.communication.disconnect_peer(peer_id);
    }
}