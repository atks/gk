use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="partition", about="partition 2 files", version="1.0")]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t=1)]
    count: u8,
}


fn main() {
    let args = Cli::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name)
    }
}
