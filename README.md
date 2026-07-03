# Huffman Coding file_compressor

A Huffman coding file compressor written in Rust.

Huffman coding is a lossless data compression algorithm that assigns variable-length codes to input symbols based on their frequencies. More frequent symbols get shorter codes, minimizing the total number of bits needed to represent the data. The codes are generated from a binary tree (Huffman tree) built bottom-up by repeatedly combining the two least frequent symbols. Huffman coding is provably optimal for symbol-by-symbol encoding with a known frequency distribution.

## Usage

```
# Compress
cargo run -- --compress --input <file> --output <file.huff>

# Decompress
cargo run -- --decompress --input <file.huff> --output <file>
```

## How it works

1. **Count frequencies** — Scans the input file and counts how many times each byte appears.
2. **Build Huffman tree** — Constructs a binary tree where more frequent bytes get shorter codes.
3. **Generate codes** — Traverses the tree to assign each byte a unique variable-length binary code.
4. **Compress** — Writes a frequency table header, then replaces each byte with its Huffman code, packing multiple codes into bytes.
5. **Decompress** — Reads the header, rebuilds the exact same tree, then walks it bit-by-bit to recover the original bytes.

## File format

```
[4 bytes]  map_len        — number of unique byte entries (u32 LE)
[9 bytes × map_len]       — each entry: [1 byte value] + [8 byte frequency u64 LE]
[remaining bytes]         — Huffman-encoded bitstream, padded to byte boundary
```

## Advantages & Benefits

- **Lossless** — Every bit of the original file is perfectly reconstructed.
- **Optimal symbol coding** — Huffman codes are provably optimal for byte-by-byte encoding given a frequency table.
- **Simple header** — Only stores the frequency table; the compressor and decompressor derive the tree independently.
- **Streaming-friendly** — Processes input in 8KB chunks, uses `BufReader`/`BufWriter` for efficiency, no need to load the entire file into memory.
- **Single-pass decompression** — Decompression is a single sequential read with constant tree lookups.
- **Good for text** — English text, source code, logs, and structured data typically achieve 40–60% compression.
- **No external dependencies for core logic** — Pure Rust standard library.

## Limitations

- **No byte-pair / dictionary modeling** — Operates on individual bytes, so it can't exploit patterns longer than 1 byte. Algorithms like LZ77 (gzip, deflate) often perform better on structured data.
- **Header overhead** — 9 bytes per unique byte. Small files or files with many unique bytes (>200) see reduced savings.
- **Ineffective on already-compressed data** — Images, audio, video, archives, or encrypted data typically have near-uniform byte distributions, yielding little to no compression.
- **Single-byte edge case** — Files with only one unique byte compress to header-only size (~13 bytes).
- **No streaming compression** — Requires a full frequency pass before encoding (two file passes total).

## Performance

For the included test file (114 KB lorem ipsum, 31 unique bytes):

- **Entropy**: ~3.99 bits/byte
- **Theoretical compressed size**: ~57 KB
- **Expected savings**: ~50%

## Building

```
cargo build --release
```

The binary will be at `target/release/file_compressor`.

## References

- Huffman, D. A. (1952). "A Method for the Construction of Minimum-Redundancy Codes". *Proceedings of the IRE*, 40(9), 1098–1101.
- [Abdul Bari Sir YT Video Explaing Huffman Coding](https://www.youtube.com/watch?v=co4_ahEDCho)
