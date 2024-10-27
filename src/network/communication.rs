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
}
