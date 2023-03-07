use clap::Parser;
use passman::generate_pass;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    length: usize,
}

fn main() {
    let args = Args::parse();
    let s = generate_pass(args.length);

    println!("Password: {}", s);
}
