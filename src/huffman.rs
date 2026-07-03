use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Bytes, Read};
use std::path::PathBuf;

// count frequency of each individual bytes existing in the og file
pub fn count_frequencies(input_path: &PathBuf) -> io::Result<HashMap<u8, usize>> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    let mut frequency = HashMap::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_rd = reader.read(&mut buffer)?;
        if bytes_rd == 0 {
            break;
        }
        for &byte in &buffer[..bytes_rd] {
            *frequency.entry(byte).or_insert(0) += 1;
        }
    }
    Ok(frequency)
}
