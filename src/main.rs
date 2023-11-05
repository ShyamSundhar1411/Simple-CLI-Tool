use  clap::Parser;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
#[derive(Parser, Debug)]
struct Cli{
    // Pattern to match in the file
    #[arg(short,long)]
    pattern: String,

    // Path of the File
    #[arg(short,long)]
    file: std::path::PathBuf,
    
    // Mode of the file
    #[arg(short, long, default_value_t = String::from("r"))]
    mode: String,
}
fn main() {
    let args = Cli::parse();
    let file_name = args.file;
    let data = args.pattern;
    let mode = args.mode;
    
    if mode == "r"{
        let lines = read_to_string(file_name).expect("Cannot Read file");
        println!("{}",lines);
    }
    else if mode == "w"{
        let mut data_file = File::create(file_name).expect("Creation Failed");
        data_file.write(data.as_bytes()).expect("Write Failed");
        println!("Created file succesfully");
    }
    else{
        panic!("Invalid mode specified");
    }
}
