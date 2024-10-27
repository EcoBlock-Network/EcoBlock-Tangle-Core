use super::peer::Peer;
use super::message::Message;

#[derive(Debug)]

pub struct communication{
    connected_peers: Vec<Peer>
}
