#![cfg(target_os = "macos")]

use portkiller_core::scan_ports;

#[test]
fn scan_ports_smoke() {
    scan_ports().expect("scan ports on macOS");
}
