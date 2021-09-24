#[derive(Debug, Default, Clone)]
pub struct Payload {
    pub context: Vec<u8>,
    pub body: Vec<u8>,
}

impl ToString for Payload {
    fn to_string(&self) -> String {
        self.body.iter().map(|i| i.to_string()).collect()
    }
}
