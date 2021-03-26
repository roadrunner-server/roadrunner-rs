trait State {
    fn value() -> i64;
    fn set(value: u64);
    fn num_execs() -> u64;
    fn is_active() -> bool;
    fn register_exec();
    fn set_last_used(lu: u64);
    fn last_used() -> u64;
}

const STATE_INACTIVE: u64 = 0;
const STATE_READY: u64 = 0;
const STATE_WORKING: u64 = 0;
const STATE_INVALID: u64 = 0;
const STATE_STOPPING: u64 = 0;
const STATE_KILLING: u64 = 0;
const STATE_KILLED: u64 = 0;
const STATE_STOPPED: u64 = 0;
const STATE_ERRORED: u64 = 0;
const STATE_REMOVE: u64 = 0;

#[derive(Clone, Copy)]
pub struct WorkerState {}

impl Default for WorkerState {
    fn default() -> Self {
        WorkerState {}
    }
}

impl State for WorkerState {
    fn value() -> i64 {
        unimplemented!()
    }

    fn set(value: u64) {
        unimplemented!()
    }

    fn num_execs() -> u64 {
        unimplemented!()
    }

    fn is_active() -> bool {
        unimplemented!()
    }

    fn register_exec() {
        unimplemented!()
    }

    fn set_last_used(lu: u64) {
        unimplemented!()
    }

    fn last_used() -> u64 {
        unimplemented!()
    }
}
