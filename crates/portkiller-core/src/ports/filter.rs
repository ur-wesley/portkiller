use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

use crate::models::PortInfo;

pub fn lookup_ports(ports: &[PortInfo], targets: &[u16]) -> Vec<PortInfo> {
    let mut result = Vec::new();
    for &target in targets {
        for port in ports {
            if port.port == target {
                result.push(port.clone());
            }
        }
    }
    result
}

pub fn missing_ports(ports: &[PortInfo], targets: &[u16]) -> Vec<u16> {
    targets
        .iter()
        .copied()
        .filter(|target| !ports.iter().any(|p| p.port == *target))
        .collect()
}

pub fn fuzzy_search_ports(ports: &[PortInfo], query: &str, limit: usize) -> Vec<PortInfo> {
    let query = query.trim();
    if query.is_empty() {
        let mut all: Vec<PortInfo> = ports.to_vec();
        all.sort_by_key(|p| p.port);
        return all;
    }

    let haystacks: Vec<String> = ports.iter().map(port_haystack).collect();
    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
    let mut buf = Vec::new();

    let mut scored: Vec<(u32, &PortInfo)> = haystacks
        .iter()
        .zip(ports.iter())
        .filter_map(|(haystack, port)| {
            buf.clear();
            let utf32 = Utf32Str::new(haystack, &mut buf);
            pattern.score(utf32, &mut matcher).map(|score| (score, port))
        })
        .collect();

    scored.sort_by(|(score_a, port_a), (score_b, port_b)| {
        score_b
            .cmp(score_a)
            .then_with(|| port_a.port.cmp(&port_b.port))
    });

    let take = if limit == 0 { scored.len() } else { limit.min(scored.len()) };
    scored
        .into_iter()
        .take(take)
        .map(|(_, port)| port.clone())
        .collect()
}

fn port_haystack(port: &PortInfo) -> String {
    format!("{} {} {}", port.port, port.process_name, port.command)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ports() -> Vec<PortInfo> {
        vec![
            PortInfo {
                port: 3000,
                pid: 100,
                process_name: "node.exe".into(),
                address: "0.0.0.0:3000".into(),
                user: "user".into(),
                command: "node server.js".into(),
            },
            PortInfo {
                port: 5173,
                pid: 200,
                process_name: "node.exe".into(),
                address: "127.0.0.1:5173".into(),
                user: "user".into(),
                command: "vite".into(),
            },
            PortInfo {
                port: 8080,
                pid: 300,
                process_name: "nginx.exe".into(),
                address: "0.0.0.0:8080".into(),
                user: "user".into(),
                command: "nginx".into(),
            },
        ]
    }

    #[test]
    fn lookup_ports_preserves_target_order() {
        let ports = sample_ports();
        let found = lookup_ports(&ports, &[8080, 3000]);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].port, 8080);
        assert_eq!(found[1].port, 3000);
    }

    #[test]
    fn missing_ports_detects_absent_targets() {
        let ports = sample_ports();
        assert_eq!(missing_ports(&ports, &[3000, 9999]), vec![9999]);
        assert!(missing_ports(&ports, &[3000, 5173]).is_empty());
    }

    #[test]
    fn fuzzy_search_empty_query_returns_all_sorted_by_port() {
        let ports = sample_ports();
        let found = fuzzy_search_ports(&ports, "  ", 0);
        assert_eq!(found.len(), 3);
        assert_eq!(found[0].port, 3000);
        assert_eq!(found[2].port, 8080);
    }

    #[test]
    fn fuzzy_search_prefers_process_name_match() {
        let ports = sample_ports();
        let found = fuzzy_search_ports(&ports, "node", 0);
        assert!(!found.is_empty());
        assert!(found.iter().all(|p| p.process_name.contains("node")));
        assert_eq!(found[0].process_name, "node.exe");
    }

    #[test]
    fn fuzzy_search_prefers_port_prefix() {
        let ports = sample_ports();
        let found = fuzzy_search_ports(&ports, "517", 0);
        assert!(!found.is_empty());
        assert_eq!(found[0].port, 5173);
    }

    #[test]
    fn fuzzy_search_respects_limit() {
        let ports = sample_ports();
        let found = fuzzy_search_ports(&ports, "node", 1);
        assert_eq!(found.len(), 1);
    }
}
