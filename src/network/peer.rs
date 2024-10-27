#[derive(Debug, Clone, PartialEq)]
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




//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_peer() {
        let peer = Peer::new("peer1", "192.168.1.1:8080");
        assert_eq!(peer.id, "peer1");
        assert_eq!(peer.address, "192.168.1.1:8080");
    }
}