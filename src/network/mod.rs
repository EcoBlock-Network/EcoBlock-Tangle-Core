pub mod peer;
pub mod message;


pub struct Network {
    peers: Vec<peer::Peer>
}