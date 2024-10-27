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

    //Method to delete a peer from the list of connected peers
    pub fn disconnect_peer(&mut self, peer: Peer){
        print!("Peer disconnected: {:?}", peer);
        self.connected_peers.retain(|p| p.id != peer.id);
    }

    //Method to send a message to all connected peers
    pub fn broadcast_message(&self, sender_id: &str, content: &str){
        let message = Message::new(sender_id, content);
        print!("Broadcasting message: {:?}", message);
        for peer in &self.connected_peers{
            println!("Sending message to {}: {:?}", peer.id, message);
        }
    }
}
