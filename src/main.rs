mod ast;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    output_file: Option<String>,

    input_files: Vec<String>,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let output_file = match args.output_file {
        Some(f) => f,
        None => "pretex.tex".to_owned(),
    };
    let input_files = args.input_files;
    if input_files.len() == 0 {
        panic!("Input files not specified");
    }

    println!("Output file: {}", output_file);
    println!("Input file: {:?}", input_files);

    for input_file in input_files {
        let file = File::open(input_file)?;
        println!("{:#?}", file);
        let mut reader = BufReader::new(file);
        let mut file_content: String = "".to_string();
        reader.read_to_string(&mut file_content)?;
        let root = grammar::RootParser::new().parse(&file_content).unwrap();
    }
    Ok(())
}
