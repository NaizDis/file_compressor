mod huffman;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Custom Huffman compressor")]
struct Cli {
    //input file
    #[arg(short, long)]
    input: PathBuf,

    //output file
    #[arg(short, long)]
    output: PathBuf,

    #[arg(short = 'c', long, group = "mode")]
    compress: bool,

    #[arg(short = 'd', long, group = "mode")]
    decompress: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.compress {
        println!("Compressing {:?} to file {:?}", cli.input, cli.output);
        //Compressing

        match huffman::count_frequencies(&cli.input) {
            Ok(freq) => {
                println!("{:#?}", freq);
                if let Some(root) = huffman::tree_build(freq) {
                    let codes = huffman::gen_codes(&root);
                    println!("{:#?}", codes);
                } else {
                    println!("Empty File!!")
                }
            }
            Err(e) => println!("Error While Reading the File : {:?}", e),
        }
    } else if cli.decompress {
        println!("Decompressing {:?} to file {:?}", cli.input, cli.output)
    } else {
        println!("Please Specify either --compress or --decompress !!");
    }
}
