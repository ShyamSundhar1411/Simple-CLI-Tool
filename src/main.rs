use  clap::Parser;

#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    println!("{}",args.pattern);
    println!("{}",args.path.display());
}
