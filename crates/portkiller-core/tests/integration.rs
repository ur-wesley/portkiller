#![cfg(windows)]

use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use portkiller_core::{kill_port, scan_ports};

struct TestServer {
    child: Child,
    port: u16,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn start_test_server() -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let mut child = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!(
                "$l = [Net.Sockets.TcpListener]::new([Net.IPAddress]::Loopback, {port}); $l.Start(); Start-Sleep 60"
            ),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("start powershell listener");

    wait_for_listener(&mut child, port);
    TestServer { child, port }
}

fn wait_for_listener(child: &mut Child, port: u16) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(Some(status)) = child.try_wait() {
            panic!("powershell listener exited early with {:?}", status);
        }
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    panic!("listener never bound to port {port}");
}

#[test]
fn scan_and_kill_port() {
    let server = start_test_server();

    let found = scan_ports()
        .expect("scan ports")
        .into_iter()
        .any(|p| p.port == server.port);
    assert!(
        found,
        "expected powershell listener on port {}",
        server.port
    );

    let killed = kill_port(server.port, true).expect("kill port");
    assert!(killed >= 1, "expected at least one process killed");
}
