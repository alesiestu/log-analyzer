use regex::Regex;
use crate::models::LogEntry;
use std::net::IpAddr;

pub fn get_ip_and_status_codes(content: &str) -> Vec<LogEntry> {
    let ip_and_status_regex = Regex::new(r"\b((?:\d{1,3}\.){3}\d{1,3}).*?\b(\d{3})\b").unwrap();
    let mut results = Vec::new();

    for cap in ip_and_status_regex.captures_iter(content) {
        if let (Ok(ip), Ok(status_code)) = (cap[1].parse::<IpAddr>(), cap[2].parse::<u16>()) {
            results.push(LogEntry {
                ip,
                status_code
            });
        }
    }

    results
}