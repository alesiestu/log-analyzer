
mod cli;
mod parser;
mod models;
use crate::parser::get_ip_and_status_codes;
mod analyzer;



fn main() {

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
    let ip_and_status=  get_ip_and_status_codes(&content);


    // Conteggio per IP
    let result =analyzer::analyze(ip_and_status, models::AnalysisType::CountByIP);
    
    let mut sorted: Vec<_> = result.into_iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Analisi completata. Ecco i risultati ordinati per numero di richieste maggiori di 100:");
    for (key, value) in sorted {
        if value > 100 {
        println!("- {} ( {} richieste )", key, value);
        }
    }

    // Conteggio per Status Code
    let ip_and_status=  get_ip_and_status_codes(&content);
    let result =analyzer::analyze(ip_and_status, models::AnalysisType::CountByStatusCode);
    let mut sorted: Vec<_> = result.into_iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Analisi completata. Ecco i risultati ordinati per codice di stato con più di 100 occorrenze:");
    
    for (key, value) in sorted {
        if value > 100 {
        println!("- {} ( {} richieste )", key, value);
    } 
    }




   
   

    
}
