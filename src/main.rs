use clap::Parser;
use std::env;
use std::fs::File;
use std::io::Read;

use controller::hello;
mod controller;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 't', long, default_value_t)]
    txt: String,

    #[arg(short = 'l', long, default_value_t)]
    headline: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() < 3 {
        eprintln!("missing args..needs text file and a headline");
        return Ok(());
    }

    // read txt from filename passed by user
    let mut f = File::open(args.txt)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    match hello(&content, &args.headline) {
        Ok(_) => println!("{:?}", "Success - your file is in the myfiles directory"),
        Err(val) => println!("{:?} {val:?}", "Docx Error - check filepaths"),
    }
    Ok(())
}
