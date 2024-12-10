use std::fs;
use std::io;


pub fn read_file(file: &String) -> io::BufReader<fs::File> {
    let file: fs::File = fs::File::open(file)
    .unwrap_or_else(|_error: io::Error| { 
        eprintln!("Failed to read file {name}", name = file);
        std::process::exit(1);            
    });

    io::BufReader::new(file)
}

