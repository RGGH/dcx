use std::env;
use std::fs::File;
use std::io::Read;
use clap::Parser;
use controller::hello;
mod controller;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 't', long, default_value_t)]
    txt : String,

    #[arg(short = 'l', long, default_value_t)]
    headline : String,
}

fn main()->Result<(),std::io::Error>{
    let args = Args::parse();
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() < 3 {
        eprintln!("missing args..");
        return Ok(())
    }

    // read txt from filename passed by user 
    let mut f = File::open(args.txt)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let _ = hello(&content, &args.headline);
    Ok(())
}
