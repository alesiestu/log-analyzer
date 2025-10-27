use std::net::IpAddr;

#[derive(Debug,Clone)]
pub struct LogEntry {
    pub ip: IpAddr,
    pub status_code: u16,
}


pub enum AnalysisType {
    CountByStatusCode,
    CountByIP
}