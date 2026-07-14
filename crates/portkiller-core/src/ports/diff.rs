use crate::models::PortInfo;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortChanges {
    pub added: Vec<PortInfo>,
    pub removed: Vec<PortInfo>,
}

fn port_key(port: &PortInfo) -> (u16, u32) {
    (port.port, port.pid)
}

pub fn diff_ports(before: &[PortInfo], after: &[PortInfo]) -> PortChanges {
    let before_keys: std::collections::HashSet<_> = before.iter().map(port_key).collect();
    let after_keys: std::collections::HashSet<_> = after.iter().map(port_key).collect();

    let added = after
        .iter()
        .filter(|p| !before_keys.contains(&port_key(p)))
        .cloned()
        .collect();

    let removed = before
        .iter()
        .filter(|p| !after_keys.contains(&port_key(p)))
        .cloned()
        .collect();

    PortChanges { added, removed }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn port(port: u16, pid: u32) -> PortInfo {
        PortInfo {
            port,
            pid,
            process_name: "node".into(),
            address: "127.0.0.1".into(),
            user: "user".into(),
            command: "node".into(),
        }
    }

    #[test]
    fn diff_ports_detects_added_and_removed() {
        let before = vec![port(3000, 1), port(5173, 2)];
        let after = vec![port(3000, 1), port(8080, 3)];

        let changes = diff_ports(&before, &after);

        assert_eq!(changes.added, vec![port(8080, 3)]);
        assert_eq!(changes.removed, vec![port(5173, 2)]);
    }

    #[test]
    fn diff_ports_empty_when_unchanged() {
        let ports = vec![port(3000, 1), port(5173, 2)];
        let changes = diff_ports(&ports, &ports);
        assert!(changes.added.is_empty());
        assert!(changes.removed.is_empty());
    }
}
