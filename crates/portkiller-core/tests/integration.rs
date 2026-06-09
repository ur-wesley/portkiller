#![cfg(windows)]

use std::net::TcpListener;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

use portkiller_core::{kill_port, scan_ports};

fn start_test_server() -> (Child, u16) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let child = Command::new("python")
        .args(["-m", "http.server", &port.to_string(), "--bind", "127.0.0.1"])
        .spawn()
        .expect("start python http.server");

    thread::sleep(Duration::from_millis(800));
    (child, port)
}

#[test]
fn scan_and_kill_port() {
    let (mut child, port) = start_test_server();

    let found = scan_ports()
        .expect("scan ports")
        .into_iter()
        .any(|p| p.port == port);
    assert!(found, "expected python server on port {port}");

    let killed = kill_port(port, true).expect("kill port");
    assert!(killed >= 1, "expected at least one process killed");

    let _ = child.kill();
}
