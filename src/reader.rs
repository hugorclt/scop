use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Lines};

pub fn get_buffers<P>(path: &str) -> Lines<BufReader<File>>
where P: AsRef<Path>, {
    let file_opened = File::open(path);
    let file = match file_opened {
        Err(e) => panic!("Error while opening file {e}"),
        Ok(file) => file,
    };

    BufReader::new(file).lines()
}
