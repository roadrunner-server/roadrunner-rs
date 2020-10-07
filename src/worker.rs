use std::time::SystemTime;
use chrono::Utc;

trait Worker {
    fn created() -> Utc::now;
}