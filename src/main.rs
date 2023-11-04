use  clap::Parser;

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
    println!("{}",args.pattern);
    println!("{}",args.file.display());
}
