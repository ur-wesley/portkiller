mod killer;
mod scanner;

pub use killer::{kill_pid, kill_port};
pub use scanner::scan_ports;
