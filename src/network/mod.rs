pub mod peer;
pub mod message;


pub struct Network {
    pub peers: Vec<peer::Peer>
}

impl Network {
    pub fn new() -> Self {
        Network {
            peers: Vec::new()
        }
    }

    //Method to add a peer to the network
    pub fn add_peer(&mut self, peer: peer::Peer) {
        self.peers.push(peer);
    }
}