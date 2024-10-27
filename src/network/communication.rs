use super::peer::Peer;
use super::message::Message;

#[derive(Debug)]

pub struct Communication{
    connected_peers: Vec<Peer>
}

impl Communication{
    //Create a new empty communication
    pub fn new() -> Self{
        Communication{
            connected_peers: Vec::new()
        }
    }

    //Method to add a peer to the list of connected peers
    pub fn connect_peer(&mut self, peer: Peer){
        print!("Peer connected: {:?}", peer);
        self.connected_peers.push(peer);
    }
}
