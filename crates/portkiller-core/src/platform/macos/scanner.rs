use std::path::Path;
use std::process::Command;

use sysinfo::{Pid, System};

use crate::models::{AppError, PortInfo};

pub fn scan_ports() -> Result<Vec<PortInfo>, AppError> {
    let output = Command::new("lsof")
        .args(["-nP", "-iTCP", "-sTCP:LISTEN", "-F", "pcn"])
        .output()?;

    if !output.status.success() {
        return Err(AppError::Other(format!(
            "lsof failed with status {}",
            output.status
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_lsof(&stdout)
}

fn parse_lsof(output: &str) -> Result<Vec<PortInfo>, AppError> {
    let mut ports = Vec::new();
    let mut pid: u32 = 0;
    let mut command = "Unknown".to_string();
    let mut port: u16 = 0;
    let mut address = String::new();

    let mut system = System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for line in output.lines() {
        if line.is_empty() {
            continue;
        }

        match line.as_bytes().first() {
            Some(b'p') => {
                pid = line[1..].parse().unwrap_or(0);
                command = "Unknown".to_string();
            }
            Some(b'c') => {
                command = line[1..].to_string();
            }
            Some(b'n') => {
                if let Some(colon) = line.rfind(':') {
                    port = line[colon + 1..].parse().unwrap_or(0);
                    address = line[1..colon].to_string();
                }
            }
            _ => {}
        }

        if line.starts_with('n') && port > 0 && pid > 0 {
            let process_name = system
                .process(Pid::from_u32(pid))
                .map(|p| p.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| Path::new(&command).file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string());

            let command_trimmed = if command.len() > 200 {
                format!("{}...", &command[..200])
            } else {
                command.clone()
            };

            ports.push(PortInfo {
                port,
                pid,
                process_name,
                address: address.clone(),
                user: "user".to_string(),
                command: command_trimmed,
            });
            port = 0;
        }
    }

    ports.sort_by_key(|p| (p.port, p.pid));
    ports.dedup_by(|a, b| a.port == b.port && a.pid == b.pid);
    Ok(ports)
}
