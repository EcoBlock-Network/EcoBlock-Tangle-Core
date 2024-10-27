#[derive(Debug, Clone)]

pub struct Message {
    pub sender_id: String,
    pub content: String
}

impl Message {
    pub fn new(sender_id: &str, content: &str) -> Self {
        Message {
            sender_id: sender_id.to_string(),
            content: content.to_string()
        }
    }
}


//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_message() {
        let message = Message::new("peer1", "Hello, world!");
        assert_eq!(message.sender_id, "peer1");
        assert_eq!(message.content, "Hello, world!");
    }
}
