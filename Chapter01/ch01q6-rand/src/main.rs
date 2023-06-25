use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    //println!("Generating random numbers");
    //for _ in 0..10 {
    //    let random_numbers: i32 = rng.gen_range(100..=400);
    //    println!("{}", random_numbers)
    //}

    println!("Generating squared ints");
    let mut squares: Vec<i32> = Vec::new();
    for i in 1..=200 {
        let sq : i32 = i * i;
        squares.push(sq);
        println!("{}", sq);
    }
}
