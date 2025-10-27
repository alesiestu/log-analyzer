use crate::models::{LogEntry,AnalysisType};
use std::collections::HashMap;



pub fn analyze(entries: &Vec<LogEntry>, analysis_type: AnalysisType) -> Vec<(String, usize)> {
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
        AnalysisType::CountByTimestamp => {
            for entry in entries {
                let timestamp_str = entry.timestamp.clone();
                *analysis_result.entry(timestamp_str).or_insert(0) += 1;
            }
        }
    }

    // Convertiamo la HashMap in un Vec e ordiniamo
    let mut sorted: Vec<_> = analysis_result.into_iter().collect();
    // Ordiniamo per conteggio (decrescente)
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

