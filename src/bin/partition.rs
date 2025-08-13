use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name="partition", about="partition 2 files", version="1.0")]
struct Cli {
    #[arg(value_name = "FILE_A",
          value_hint = ValueHint::FilePath,
          value_parser = clap::value_parser!(PathBuf))]
    file_a: PathBuf,

    #[arg(value_name = "FILE_B",
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf))]
    file_b: PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Hello, {} {}!", args.file_a.display(), args.file_b.display())

}
