extern crate oorandom;

fn main() {
    let some_seed = 4;
    let mut rng = oorandom::Rand64::new(some_seed);
    println!("Your random number is: {:x}", rng.rand_u64());
    println!("Your random number is: {:x}", rng.rand_u64());
    println!("Your random number is: {:x}", rng.rand_u64());
    println!("Your random number is: {:x}", rng.rand_u64());
    println!("Your random number is: {:x}", rng.rand_u64());
    println!("Your random number is: {:x}", rng.rand_u64());
}