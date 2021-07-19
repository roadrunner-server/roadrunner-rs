use crate::errors::Error;
use crate::frame::Frame;
use crate::relay::Relay;

struct Pipes {}

impl Relay<Frame> for Pipes {
    fn send(&mut self, frame: &mut Frame) -> Result<usize, Error> {
        todo!()
    }

    fn receive(&mut self) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn receive_string(&mut self) -> Result<String, Error> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}
