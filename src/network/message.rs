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