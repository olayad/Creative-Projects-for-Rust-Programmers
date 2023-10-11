use rand::Rng;

fn main() {
    println!("Hello, rand!");
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let random_numbers: f32 = rng.gen_range(100.0..=400.0);
        println!("{}", random_numbers)
    }


}
