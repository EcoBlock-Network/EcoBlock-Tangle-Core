#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: String
}

impl Peer {
    pub fn new(id: &str, address: &str) -> Self {
        Peer {
            id : id.to_string(),
            address : address.to_string()
        }
    }
}