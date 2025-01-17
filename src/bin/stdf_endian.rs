use std::fs::File;
use std::io;
use clap::Parser;
use stdf_rs::get_endianness_from_file;

#[derive(Parser)]
#[clap(
    name = "STDF Endian Checker",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "Checks the endianness of an STDF file",
    long_about = include_str!("../../README.md")
)]
struct Cli {
    /// The input file to check
    input: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let mut file = File::open(args.input)?;

    match get_endianness_from_file(&mut file)? {
        Some(endian) => match endian {
            stdf_rs::Endian::Little => println!("LE"),
            stdf_rs::Endian::Big => println!("BE"),
        },
        None => println!("NA"),
    }

    Ok(())
}
