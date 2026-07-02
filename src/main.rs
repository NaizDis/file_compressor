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
        println!("Compressing {:?} to file {:?}", cli.input, cli.output)
        //Compressing
    } else if cli.decompress {
        println!("Decompressing {:?} to file {:?}", cli.input, cli.output)
    } else {
        println!("Please Specify either --compress or --decompress !!");
    }
}
