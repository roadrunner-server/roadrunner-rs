use crate::errors::Error;
use crate::state::WorkerState;
use goridge_rs::relay::Relay;
use log::{debug, info, trace, warn};
use std::cell::Cell;
use std::io::{stdout, BufReader, BufWriter, ErrorKind, Read, Write};
use std::process::{Child, ExitStatus, Stdio};
use std::process::{ChildStderr, ChildStdin, ChildStdout, Command};
use std::time::Instant;

pub trait Worker<T: Relay<T>> {
    // time in unix nano format
    fn created(&mut self) -> std::time::Instant;

    // channel WorkerEvents
    fn events(&self);

    fn pid(&self) -> u16;

    fn state(&self) -> WorkerState;

    fn start(&mut self) -> Result<(), Error>;

    fn wait(&mut self) -> Result<ExitStatus, Error>;

    fn exec(&mut self) -> Result<(), Error>;

    fn exec_ttl(&mut self) -> Result<(), Error>;

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

struct ChildProcess {
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<Option<ChildStdout>>,
    stderr: BufReader<Option<ChildStderr>>,
}

pub struct WorkerProcess<T: Relay<T>> {
    created: std::time::Instant,
    // events channel
    state: WorkerState,
    pid: u16,
    child: Child,
    child_fds: Option<ChildProcess>,
    relay: T,
}

impl<T: Relay<T>> WorkerProcess<T> {
    fn new(rl: T, cmd: &str) -> Result<Self, Error> {
        debug!("worker_created");

        let mut cc = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(WorkerProcess {
            created: Instant::now(),
            state: WorkerState::default(),
            // cmd: Cell::new(cc),
            child_fds: None,
            pid: 0,
            relay: rl,
            child: cc,
        })
    }
}

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> std::io::Result<u64>
where
    R: std::io::Read,
    W: std::io::Write,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;

    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

impl<T> Worker<T> for WorkerProcess<T>
where
    T: Relay<T>,
{
    fn created(&mut self) -> std::time::Instant {
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

    fn start(&mut self) -> Result<(), Error> {
        // let spawned = self.cmd.spawn()?;
        // self.child = Option::from(spawned);
        Ok(())
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

    fn exec(&mut self) -> Result<(), Error> {
        // self.child.
        todo!()
    }

    fn exec_ttl(&mut self) -> Result<(), Error> {
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
