use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let s = generate_pass(10);

    println!("Password: {}", s);
}

fn generate_pass(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
