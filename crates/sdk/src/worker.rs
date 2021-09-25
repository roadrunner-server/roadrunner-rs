use crate::errors::Error;
use crate::payload::Payload;
use crate::state::WorkerState;
use goridge_rs::relay::Relay;
use log::{debug, info, trace, warn};
use std::process::Command;
use std::process::{Child, ExitStatus, Stdio};
use std::time::Instant;

pub trait Worker<T: Relay<T>> {
    // time in unix nano format
    fn created(&mut self) -> std::time::Instant;
    // channel WorkerEvents
    fn events(&self);
    fn pid(&self) -> u16;
    fn state(&self) -> WorkerState;
    fn wait(&mut self) -> Result<ExitStatus, Error>;
    fn exec(&mut self, p: Payload) -> Result<(), Error>;
    fn exec_ttl(&mut self, p: Payload) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
    fn kill(&self) -> Result<(), Error>;
    fn attach_relay(&mut self, rl: T);

    // state
    fn value() -> i64;
    fn set(value: u64);
    fn num_execs() -> u64;
    fn is_active() -> bool;
    fn register_exec();
    fn set_last_used(&mut self, lu: u64);
    fn last_used() -> u64;
}

pub struct WorkerProcess<T: Relay<T>> {
    created: std::time::Instant,
    state: WorkerState,
    pid: u16,
    child: Child,
    relay: T,
}

impl<T: Relay<T>> WorkerProcess<T> {
    fn new(rl: T, cmd: &str) -> Result<Self, Error> {
        debug!("worker_created");

        let cc = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(WorkerProcess {
            created: Instant::now(),
            state: WorkerState::default(),
            pid: 0,
            relay: rl,
            child: cc,
        })
    }
}

impl<T> Worker<T> for WorkerProcess<T>
where
    T: Relay<T>,
{
    fn created(&mut self) -> std::time::Instant {
        debug!("created called: {:?}", self.created);
        self.created
    }

    // channel with events???
    fn events(&self) {
        unimplemented!()
    }

    fn pid(&self) -> u16 {
        self.pid
    }

    fn state(&self) -> WorkerState {
        self.state
    }

    fn wait(&mut self) -> Result<ExitStatus, Error> {
        debug!("wait child process");
        let res = self.child.wait();
        return match res {
            Ok(rr) => Ok(rr),
            Err(rr) => {
                eprintln!("return with error: {}", rr);
                Err(Error::WaitError {
                    cause: rr.to_string(),
                })
            }
        };
    }

    fn exec(&mut self, p: Payload) -> Result<(), Error> {
        assert!(p.body.len() > 0);
        assert!(p.context.len() > 0);

        Ok(())
    }

    fn exec_ttl(&mut self, p: Payload) -> Result<(), Error> {
        todo!()
    }

    fn stop(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn kill(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn attach_relay(&mut self, rl: T) {
        self.relay = rl;
    }

    fn value() -> i64 {
        todo!()
    }

    fn set(value: u64) {
        todo!()
    }

    fn num_execs() -> u64 {
        todo!()
    }

    fn is_active() -> bool {
        todo!()
    }

    fn register_exec() {
        todo!()
    }

    fn set_last_used(&mut self, lu: u64) {
        todo!()
    }

    fn last_used() -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::worker::WorkerProcess;

    #[test]
    fn test_init_worker() {
        // WorkerProcess::new(goridge_rs::pipe::Pipes {}, "test");
    }
    // use super::*;
    // use goridge_rs::pipe::PipeRelay;
    //
    // #[test]
    // fn test_init_worker() {
    //     let command = Command::new("ls");
    //     let pipe_relay = PipeRelay::new_relay(None, None);
    //     let mut worker = WorkerProcess::new(pipe_relay, command);
    //     let a = worker.start();
    //     let b = worker.wait();
    // }
}
