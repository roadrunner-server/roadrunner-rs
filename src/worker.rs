use crate::state::WorkerState;

trait Worker {
    // time in unix nano format
    fn created() -> u64;

    // channel WorkerEvents
    fn events();

    fn pid() -> u16;

    fn state() -> WorkerState;


}


