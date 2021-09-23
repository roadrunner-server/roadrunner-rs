/// State represents current state of the worker

trait State {
    fn value() -> i64;
    fn set(value: u64);
    fn num_execs() -> u64;
    fn is_active() -> bool;
    fn register_exec();
    fn set_last_used(lu: u64);
    fn last_used() -> u64;
}

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
