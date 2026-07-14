mod diff;
mod filter;

pub use diff::{diff_ports, PortChanges};
pub use filter::{fuzzy_search_ports, lookup_ports, missing_ports};
pub use crate::platform::{kill_pid, kill_port, scan_ports};
