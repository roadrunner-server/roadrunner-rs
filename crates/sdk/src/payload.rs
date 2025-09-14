use std::fmt::Display;
use std::fmt::Formatter;

struct Test {}

impl Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Payload {
    pub codec: u8,
    pub context: Vec<u8>,
    pub body: Vec<u8>,
}

impl Payload {
    pub fn new<C: Into<Vec<u8>>>(codec: u8, context: C, body: C) -> Self {
        Self {
            codec,
            context: context.into(),
            body: body.into(),
        }
    }
}
