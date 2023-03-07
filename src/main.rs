use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};

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

fn generate_pass(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
