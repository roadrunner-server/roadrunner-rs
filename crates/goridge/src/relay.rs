use crate::errors::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Relay<T> {
    fn send(&mut self, frame: &mut T) -> Result<(), Error>;
    fn receive(&mut self) -> Result<Vec<u8>, Error>;
    fn receive_string(&mut self) -> Result<String, Error>;
    fn close(&self);
}
