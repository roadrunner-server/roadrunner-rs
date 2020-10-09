use crate::state::WorkerState;
use crate::errors::Error;
use goridge_rs::relay::Relay;
use std::time::Instant;

trait Worker<T: Relay> {
    // time in unix nano format
    fn created(&self) -> u64;

    // channel WorkerEvents
    fn events(&self);

    fn pid(&self) -> u16;

    fn state(&self) -> WorkerState;

    fn wait(&self) -> Result<(), Error>;

    fn stop(&self) -> Result<(), Error>;

    fn kill(&self) -> Result<(), Error>;

    fn relay(&self) -> T;

    fn attach_relay(&mut self, rl: T);
}

struct WorkerProcess<T: Relay + Copy> {
    created: std::time::Instant,
    // events channel
    state: WorkerState,
    cmd: std::process::Command,
    pid: i32,
    // errbuffer
    // endstate
    relay: T,
}

impl<T: Relay + Copy> WorkerProcess<T> {
    fn new(rl: T) -> Self {
        WorkerProcess {
            created: Instant::now(),
            state: WorkerState::default(),
            cmd: std::process::Command::new(""),
            pid: 0,
            relay: rl,
        }
    }
}

impl<T> Worker<T> for WorkerProcess<T> where T: Relay + Copy {
    fn created(&self) -> u64 {
        unimplemented!()
    }

    fn events(&self) {
        unimplemented!()
    }

    fn pid(&self) -> u16 {
        unimplemented!()
    }

    fn state(&self) -> WorkerState {
        unimplemented!()
    }

    fn wait(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn stop(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn kill(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn relay(&self) -> T {
        self.relay
    }

    fn attach_relay(&mut self, rl: T) {
        self.relay = rl;
    }
}
