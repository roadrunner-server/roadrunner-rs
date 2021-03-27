use crate::errors::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Relay {
    fn send(&mut self, data: &[u8], flags: u8) -> Result<usize, Error>;
    fn receive(&mut self) -> Result<Vec<u8>, Error>;
    fn receive_string(&mut self) -> Result<String, Error>;
    fn close(&self);
}
