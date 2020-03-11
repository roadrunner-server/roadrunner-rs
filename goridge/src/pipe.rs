#![allow(dead_code)]

use std::process::{ChildStdin, ChildStdout, Command, Stdio};

trait Relay {
    fn send(data: &[u8], flags: u8);
    fn receive(data: &[u8]);
    fn close();
}

// TODO send raw fd?
const BUFFER_SIZE: u64 = 655336;

struct PipeRelay {
    buf_size: u64,
    child_stdin: Option<ChildStdin>,
    child_stdout: Option<ChildStdout>,
}

impl PipeRelay {
    pub fn new_relay(ch_std_in: Option<ChildStdin>, ch_std_out: Option<ChildStdout>) -> Self {
        PipeRelay {
            buf_size: BUFFER_SIZE,
            child_stdin: ch_std_in,
            child_stdout: ch_std_out,
        }
    }
}

impl Relay for PipeRelay {
    fn send(data: &[u8], flags: u8) {
        unimplemented!()
    }

    fn receive(data: &[u8]) {
        unimplemented!()
    }

    fn close() {
        unimplemented!()
    }
}
