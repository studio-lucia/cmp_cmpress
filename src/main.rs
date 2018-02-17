use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::exit;

#[macro_use] extern crate quicli;
extern crate sega_cmp;

use quicli::prelude::*;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "t", default_value = "8",
                help = "Compression type (one of 8, 16, 32)")]
    compression_size: u8,
    #[structopt(short = "f", default_value = "0",
                help = "Byte offset in input file to begin compression")]
    offset: u64,
    #[structopt(short = "s",
                help = "Maximum number of bytes to compress")]
    max_bytes: Option<usize>,
    #[structopt(help = "Input file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(help = "Output file", parse(from_os_str))]
    output: PathBuf,
}

main!(|args: Opt| {
    let size;
    match args.compression_size {
        8  => size = sega_cmp::Size::Byte,
        16 => size = sega_cmp::Size::Word,
        32 => size = sega_cmp::Size::Longword,
        _  => {
            println!("Compression size must be one of 8, 16, or 32! (Provided: {})",
                     args.compression_size);
            exit(1);
        }
    }

    if !args.input.exists() {
        println!("Input file {} does not exist!", args.input.to_str().unwrap());
        exit(1);
    }

    let mut input_file;
    match File::open(&args.input) {
        Ok(f) => input_file = f,
        Err(e) => {
            println!("Error while trying to open file {}: {}", args.input.to_str().unwrap(), e);
            exit(1);
        }
    }
    // Since this defaults to 0 we can use it unconditionally
    input_file.seek(SeekFrom::Start(args.offset))?;

    let mut input_data = vec![];
    match args.max_bytes {
        Some(n) => {
            // If we had a request to read N bytes, make sure we only read that number
            input_data.reserve(n);
            assert_eq!(input_data.capacity(), n);
            input_file.read_exact(&mut input_data)?;
        }
        None => {
            input_file.read_to_end(&mut input_data)?;
        }
    }

    let output_header = sega_cmp::create_header(input_data.len() as i32, size);
    let output_data = sega_cmp::compress(&input_data, size)?;

    let mut output_file;
    match File::create(&args.output) {
        Ok(f) => output_file = f,
        Err(e) => {
            println!("Error writing to file {}: {}", args.output.to_str().unwrap(), e);
            exit(1);
        }
    }
    output_file.write_all(&output_header)?;
    output_file.write_all(&output_data)?;
});
