use super::peer::Peer;
use super::message::Message;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;

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
    pub fn disconnect_peer(&mut self, peer_id: &str){
            print!("Peer disconnected: {:?}", peer_id);
            self.connected_peers.retain(|p| p.id != peer_id);
        }

    //Method to send a message to all connected peers
    pub fn broadcast_message(&self, sender_id: &str, content: &str){
        let message = Message::new(sender_id, content);
        print!("Broadcasting message: {:?}", message);
        for peer in &self.connected_peers{
            println!("Sending message to {}: {:?}", peer.id, message);
        }
    }

    //Method to start a TCP server for accepting incoming connections
    pub async fn start_server(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address).await?;
        print!("Server listening on {}", address);

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(Self::handle_connection(socket));
        }
        }
    

    //Method to handle incoming connections
    async fn handle_connection(mut socket : TcpStream){
        let mut buffer = [0u8; 1024];

        loop {
            let bytes_read = socket.read(&mut buffer).await.unwrap();

            if bytes_read == 0 {
                break;
            }

            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            print!("Received message: {:?}", message);

            //Echo the message back to the sender
            socket.write_all(&buffer[..bytes_read]).await.unwrap();
        }
    } 
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::peer::Peer;

    #[test]
    fn test_connect_peer() {
        let mut comm = Communication::new();
        let peer = Peer::new("peer1", "192.168.1.1:8080");
        comm.connect_peer(peer.clone());
        assert_eq!(comm.connected_peers.len(), 1);
        assert_eq!(comm.connected_peers[0], peer);
    }

    #[test]
    fn test_disconnect_peer() {
        let mut comm = Communication::new();
        let peer = Peer::new("peer1", "192.168.1.1:8080");
        comm.connect_peer(peer);
        comm.disconnect_peer("peer1");
        assert!(comm.connected_peers.is_empty());
    }

    #[test]
    fn test_broadcast_message() {
        let mut comm = Communication::new();
        let peer1 = Peer::new("peer1", "192.168.1.1:8080");
        let peer2 = Peer::new("peer2", "192.168.1.2:8080");
        comm.connect_peer(peer1);
        comm.connect_peer(peer2);
        
        // Pour ce test, on va simplement vérifier que l'envoi de message ne provoque pas d'erreur
        comm.broadcast_message("peer1", "Hello from peer1!");
    }
}
