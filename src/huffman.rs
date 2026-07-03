use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub byte: Option<u8>,
    pub freq: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct Bitwriter<W: Write> {
    writer: W,
    curr_byte: u8,
    bit_count: u8,
}

impl<W: Write> Bitwriter<W> {
    pub fn new(writer: W) -> Self {
        Bitwriter {
            writer,
            curr_byte: 0,
            bit_count: 0,
        }
    }

    //this function write a single bit to internal byte buffer
    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        //shift bits left
        self.curr_byte <<= 1;

        if bit {
            self.curr_byte |= 1;
        }
        self.bit_count += 1;

        if self.bit_count == 8 {
            self.writer.write_all(&[self.curr_byte])?;
            self.curr_byte = 0;
            self.bit_count = 0;
        }
        Ok(())
    }

    pub fn write_code(&mut self, code: &str) -> io::Result<()> {
        for ch in code.chars() {
            self.write_bit(ch == '1')?;
        }
        Ok(())
    }

    pub fn clean_left(&mut self) -> io::Result<()> {
        if self.bit_count > 0 {
            self.curr_byte <<= 8 - self.bit_count;
            self.writer.write_all(&[self.curr_byte])?;

            self.curr_byte = 0;
            self.bit_count = 0;
        }
        self.writer.flush()
    }
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

pub fn tree_build(frequencies: &HashMap<u8, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (&byte, &freq) in frequencies {
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

//recursive code genrator
pub fn gen_codes(root: &Node) -> HashMap<u8, String> {
    let mut codes = HashMap::new();
    traversal(root, String::new(), &mut codes);
    codes
}

fn traversal(node: &Node, curr_code: String, codes: &mut HashMap<u8, String>) {
    //leaf nodes only hold the actual bytes rest are parent
    if let Some(byte) = node.byte {
        codes.insert(byte, curr_code);
    } else {
        //Parent Node Move on recursive
        if let Some(ref left_c) = node.left {
            let mut left_code = curr_code.clone();
            left_code.push('0');
            traversal(left_c, left_code, codes);
        }
        if let Some(ref right_c) = node.right {
            let mut right_code = curr_code.clone();
            right_code.push('1');
            traversal(right_c, right_code, codes);
        }
    }
}

pub fn compress(
    input_path: &PathBuf,
    output_path: &PathBuf,
    codes: &HashMap<u8, String>,
    frequencies: &HashMap<u8, usize>,
) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let mut output_file = File::create(output_path)?;

    // store number of unique entries in compressed file
    let map_len = frequencies.len() as u32;
    output_file.write_all(&map_len.to_le_bytes())?;

    //write each byte and its frequencies
    for (&byte, &freq) in frequencies {
        output_file.write_all(&[byte])?;

        //write freq of the byre as little endian bytes in file
        let fre_u64 = freq as u64;
        output_file.write_all(&fre_u64.to_le_bytes())?;
    }

    //Bit writer convert 8 bits to byte chunk
    //bufwriter convert those bytes to 8KB chunks and write on hard drive
    let mut bit_writer = Bitwriter::new(BufWriter::new(output_file));
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        for &byte in &buffer[..bytes_read] {
            if let Some(code) = codes.get(&byte) {
                bit_writer.write_code(code)?;
            }
        }
    }
    bit_writer.clean_left()?;

    Ok(())
}
