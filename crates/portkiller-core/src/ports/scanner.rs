use std::collections::HashMap;
use std::mem;
use std::net::{Ipv4Addr, Ipv6Addr};

use sysinfo::{Pid, System};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, MIB_TCP6ROW_OWNER_PID, MIB_TCPROW_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
};
use windows_sys::Win32::Networking::WinSock::{AF_INET, AF_INET6};

use crate::models::{AppError, PortInfo};

const MIB_TCP_STATE_LISTEN: u32 = 2;

pub fn scan_ports() -> Result<Vec<PortInfo>, AppError> {
    let mut ports = Vec::new();
    let mut process_cache: HashMap<u32, ProcessDetails> = HashMap::new();
    let mut system = System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for row in read_tcp_v4()? {
        if row.dwState != MIB_TCP_STATE_LISTEN {
            continue;
        }
        let port = decode_port(row.dwLocalPort);
        let pid = row.dwOwningPid as u32;
        let address = Ipv4Addr::from(row.dwLocalAddr.to_le_bytes()).to_string();
        if let Some(info) = build_port_info(port, pid, address, &mut process_cache, &system) {
            ports.push(info);
        }
    }

    for row in read_tcp_v6()? {
        if row.dwState != MIB_TCP_STATE_LISTEN {
            continue;
        }
        let port = decode_port(row.dwLocalPort);
        let pid = row.dwOwningPid as u32;
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&row.ucLocalAddr[..16]);
        let address = Ipv6Addr::from(bytes).to_string();
        if let Some(info) = build_port_info(port, pid, address, &mut process_cache, &system) {
            ports.push(info);
        }
    }

    ports.sort_by_key(|p| (p.port, p.pid));
    ports.dedup_by(|a, b| a.port == b.port && a.pid == b.pid);
    Ok(ports)
}

struct ProcessDetails {
    name: String,
    command: String,
    user: String,
}

fn build_port_info(
    port: u16,
    pid: u32,
    address: String,
    cache: &mut HashMap<u32, ProcessDetails>,
    system: &System,
) -> Option<PortInfo> {
    if pid == 0 {
        return None;
    }
    let details = cache
        .entry(pid)
        .or_insert_with(|| get_process_details(pid, system));
    Some(PortInfo {
        port,
        pid,
        process_name: details.name.clone(),
        address,
        user: details.user.clone(),
        command: details.command.clone(),
    })
}

fn get_process_details(pid: u32, system: &System) -> ProcessDetails {
    let name = system
        .process(Pid::from_u32(pid))
        .map(|p| p.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".into());

    let command = get_command_line(pid).unwrap_or_else(|| name.clone());
    let command = if command.len() > 200 {
        format!("{}...", &command[..200])
    } else {
        command
    };

    let user = std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".into());

    ProcessDetails {
        name,
        command,
        user,
    }
}

fn get_command_line(pid: u32) -> Option<String> {
    use std::collections::HashMap;
    use wmi::{COMLibrary, WMIConnection};

    let com = COMLibrary::new().ok()?;
    let wmi = WMIConnection::new(com).ok()?;
    let query = format!("SELECT CommandLine FROM Win32_Process WHERE ProcessId = {pid}");
    let results: Vec<HashMap<String, wmi::Variant>> = wmi.raw_query(&query).ok()?;
    results.first().and_then(|row| {
        row.get("CommandLine").and_then(|v| match v {
            wmi::Variant::String(s) => Some(s.clone()),
            wmi::Variant::Null => None,
            other => Some(format!("{other:?}")),
        })
    })
}

fn decode_port(raw: u32) -> u16 {
    let bytes = raw.to_le_bytes();
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn read_tcp_v4() -> Result<Vec<MIB_TCPROW_OWNER_PID>, AppError> {
    let mut size: u32 = 0;
    let status = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            1,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if status != 0 && size == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0u8; size as usize];
    let status = unsafe {
        GetExtendedTcpTable(
            buffer.as_mut_ptr().cast(),
            &mut size,
            1,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if status != 0 {
        return Err(AppError::WindowsApi(format!(
            "GetExtendedTcpTable v4 failed: {status}"
        )));
    }

    let num_entries = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) as usize;
    let row_size = mem::size_of::<MIB_TCPROW_OWNER_PID>();
    let mut rows = Vec::with_capacity(num_entries);
    let mut offset = 4usize;
    for _ in 0..num_entries {
        if offset + row_size > buffer.len() {
            break;
        }
        let row = unsafe { *(buffer.as_ptr().add(offset) as *const MIB_TCPROW_OWNER_PID) };
        rows.push(row);
        offset += row_size;
    }
    Ok(rows)
}

fn read_tcp_v6() -> Result<Vec<MIB_TCP6ROW_OWNER_PID>, AppError> {
    let mut size: u32 = 0;
    let status = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            1,
            AF_INET6 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if status != 0 && size == 0 {
        return Ok(Vec::new());
    }

    let mut buffer = vec![0u8; size as usize];
    let status = unsafe {
        GetExtendedTcpTable(
            buffer.as_mut_ptr().cast(),
            &mut size,
            1,
            AF_INET6 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if status != 0 {
        return Ok(Vec::new());
    }

    let num_entries = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) as usize;
    let row_size = mem::size_of::<MIB_TCP6ROW_OWNER_PID>();
    let mut rows = Vec::with_capacity(num_entries);
    let mut offset = 4usize;
    for _ in 0..num_entries {
        if offset + row_size > buffer.len() {
            break;
        }
        let row = unsafe { *(buffer.as_ptr().add(offset) as *const MIB_TCP6ROW_OWNER_PID) };
        rows.push(row);
        offset += row_size;
    }
    Ok(rows)
}
