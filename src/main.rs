
mod cli;
use regex::Regex;

fn main() {

    println!("Inserisci il path del file di log come argomento della riga di comando.");
    let path = std::env::args().nth(1).expect("no path given");
    if path.is_empty() {
        panic!("Path non può essere vuoto");
    }
    if path != "" {
        let line_count: usize = cli::read_path_from_args(&path);
        println!("Il file di log contiene {} righe.", line_count);        
    }

    let content = cli::read_content_from_path(path);
    let ip_and_status_regex = Regex::new(r"\b((?:\d{1,3}\.){3}\d{1,3}).*?\b(\d{3})\b").unwrap();
    for cap in ip_and_status_regex.captures_iter(&content) {
        println!("{} {}", &cap[1], &cap[2]);      
    }

    
}
