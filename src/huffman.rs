use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub byte: Option<u8>,
    pub freq: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare other to self to get a max heap ,
        // compare self to other resturn a min heap
        other.freq.cmp(&self.freq)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

pub fn tree_build(frequencies: HashMap<u8, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (byte, freq) in frequencies {
        heap.push(Node {
            byte: Some(byte),
            freq,
            left: None,
            right: None,
        });
    }
    //File exmpty
    if heap.is_empty() {
        return None;
    }

    //building the tree bottom up till one node remain
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let parent = Node {
            byte: None,
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        heap.push(parent);
    }
    heap.pop().map(Box::new)
}
