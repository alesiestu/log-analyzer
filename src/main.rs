
mod cli;
mod parser;
mod models;
use crate::parser::get_ip_and_status_codes;
mod analyzer;




#[tokio::main]
async fn main() {
    println!("Inserisci il path del file di log come argomento della riga di comando.");
    
    let path = std::env::args().nth(1).expect("no path given");
    if path.is_empty() {
        panic!("Path non può essere vuoto");
    }
    if path != "" {
        let line_count: usize = cli::read_path_from_args(&path);
        println!("Il sistema ha analizzato {} righe.", line_count);        
    }

    let content = cli::read_content_from_path(path);
    let ip_and_status = get_ip_and_status_codes(&content).await;

    for entry in &ip_and_status {
        println!("Parsed Entry: IP: {}, Status Code: {}, Timestamp: {}", entry.ip, entry.status_code, entry.timestamp);
    }


    // Conteggio per IP
    let result =analyzer::analyze(&ip_and_status, models::AnalysisType::CountByIP);
    println!("Analisi completata. Ecco i risultati ordinati per numero di richieste maggiori di 100:");
    for (key, value) in result {
        if value > 100 {
        println!("- {} ( {} richieste )", key, value);
        }
    }

    // Conteggio per Status Code
    let result =analyzer::analyze(&ip_and_status, models::AnalysisType::CountByStatusCode);

    println!("Analisi completata. Ecco i risultati ordinati per codice di stato con più di 100 occorrenze:");
    
    for (key, value) in result {
        if value > 100 {
        println!("- {} ( {} richieste )", key, value);
    } 
    }

    let result =analyzer::analyze(&ip_and_status, models::AnalysisType::CountByTimestamp);
    
    println!("Analisi completata. Ecco i risultati ordinati per timestamp con più di 100 occorrenze:");
    for (key, value) in result {
        if value > 2 {
        println!("- {} ( {} richieste )", key, value);  
    }  
    }

    
}
