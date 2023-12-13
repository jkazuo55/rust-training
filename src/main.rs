use rand::Rng;

fn main() {
    // Generate a random number between 1 and 100
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);

    println!("Random number: {}", random_number);
}
