use crate::models::{LogEntry,AnalysisType};
use std::collections::HashMap;
use std::net::IpAddr;


pub fn analyze(entries: Vec<LogEntry>, analysis_type: AnalysisType) -> HashMap<String, usize> {
    let mut analysis_result: HashMap<String, usize> = HashMap::new();

    match analysis_type {
        AnalysisType::CountByStatusCode => {
            for entry in entries {
                let status_code_str = entry.status_code.to_string();
                *analysis_result.entry(status_code_str).or_insert(0) += 1;
            }
        }
        AnalysisType::CountByIP => {
            for entry in entries {
                let ip_str = entry.ip.to_string();
                *analysis_result.entry(ip_str).or_insert(0) += 1;
            }
        }
    }

    analysis_result
}

