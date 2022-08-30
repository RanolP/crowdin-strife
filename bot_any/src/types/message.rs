pub struct MessageOutput {
    pub content: Option<String>,
}

pub enum MessagePart {
    Text(String),
}

pub struct MessageWrite<const DONE: bool = true> {
    contents: Vec<MessagePart>,
}

impl MessageWrite<true> {
    pub fn contents(&self) -> &[MessagePart] {
        &self.contents
    }
}

impl MessageWrite<false> {
    pub fn begin() -> Self {
        MessageWrite {
            contents: Vec::new(),
        }
    }

    pub fn push_str(mut self, content: String) -> Self {
        self.contents.push(MessagePart::Text(content));
        self
    }

    pub fn end(self) -> MessageWrite<true> {
        MessageWrite {
            contents: self.contents,
        }
    }
}
