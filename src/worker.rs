use crate::state::WorkerState;
use crate::errors::Error;

trait Worker {
    // time in unix nano format
    fn created() -> u64;

    // channel WorkerEvents
    fn events();

    fn pid() -> u16;

    fn state() -> WorkerState;

    fn wait() -> Result<(), Error>;

    fn stop() -> Result<(), Error>;

    fn kill() -> Result<(), Error>;

    // relay
    // attach relay
}


