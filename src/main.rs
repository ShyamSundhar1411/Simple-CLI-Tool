use  clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
struct Cli{
    // Pattern to match in the file
    #[arg(short,long)]
    pattern: String,

    // Path of the File
    #[arg(short,long)]
    file: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let file_name = args.file;
    println!("{}",file_name.display());
    let data = args.pattern;
    let mut data_file = File::create(file_name).expect("Creation Failed");
    data_file.write(data.as_bytes()).expect("Write Failed");
    println!("Created file succesfully");
}
