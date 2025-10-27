use std::fs;

pub fn read_path_from_args(path: &String) -> usize {
    println!("Il file di log si trova in: {}", path);
    let content = fs::read_to_string(path).expect("failed to read file");
    let line_count = content.lines().count();

    line_count
}

pub fn read_content_from_path(path: String) -> String {
    fs::read_to_string(path).expect("failed to read file")
}