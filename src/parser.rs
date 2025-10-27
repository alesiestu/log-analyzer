use regex::Regex;
use crate::models::LogEntry;
use std::net::IpAddr;
use tokio::sync::mpsc;
use futures::stream::{self, StreamExt};

const CHUNK_SIZE: usize = 1000; // Linee per chunk
const MAX_CONCURRENT: usize = 10;



pub async fn get_ip_and_status_codes(content: &str) -> Vec<LogEntry> {
    let ip_and_status_regex = Regex::new(        r#"^(?P<ip>\d{1,3}(?:\.\d{1,3}){3})\s+\S+\s+\S+\s+\[(?P<ts>[^\]]+)\]\s+"[^"]+"\s+(?P<status>\d{3})\b"#
).unwrap();
    
    // Divide il contenuto in chunks di linee
    let lines: Vec<String> = content.lines().map(String::from).collect();
    let chunks: Vec<Vec<String>> = lines
        .chunks(CHUNK_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    let (tx, mut rx) = mpsc::channel(chunks.len());

    // Processa i chunks in parallelo
    stream::iter(chunks)
        .for_each_concurrent(MAX_CONCURRENT, |chunk| {
            let tx = tx.clone();
            let regex = ip_and_status_regex.clone();
            async move {
                let mut chunk_results = Vec::new();
                for line in chunk {
                    if let Some(cap) = regex.captures(&line) {
                        if let (Ok(ip), Ok(status_code)) = (cap["ip"].parse::<IpAddr>(), cap["status"].parse::<u16>()) {
                            let timestamp = cap["ts"].to_string();
                            chunk_results.push(LogEntry {
                                ip,
                                status_code,
                                timestamp
                            });
                        }
                    }
                }
                let _ = tx.send(chunk_results).await;
            }
        })
        .await;

    // Raccoglie i risultati
    let mut results = Vec::new();
    drop(tx); // Chiude il canale

    while let Some(mut chunk_results) = rx.recv().await {
        results.append(&mut chunk_results);
    }

    results
}