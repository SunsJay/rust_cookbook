use rand::Rng; // Import the Rng trait for random number generation
use rand_distr::{Normal, Distribution}; // Import necessary traits for normal distribution
use rand::thread_rng;
use algorithms::custom::Point;

// Import thread_rng for random number generation
fn main() {
    let mut rng = thread_rng(); // Create a random number generator

    let n1: u8 = rng.gen(); // Generate a random u8
    let n2: u16 = rng.gen(); // Generate a random u16
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>()); // Generate a random u32
    println!("Random i32: {}", rng.gen::<i32>()); // Generate a random i32
    println!("Random float: {}", rng.gen::<f64>()); // Generate a random f64

    // Generate random integers and floats in specific ranges
    println!("Integer: {}", rng.gen_range(0..10)); // Generate random integer in range
    println!("Float: {}", rng.gen_range(0.0..10.0)); // Generate random float in range

    // Create a normal distribution and sample from it
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng); // Sample from the normal distribution
    println!("Normal distribution sample: {:?}", v);

    let mut rng = thread_rng();
    let rand_tuple = rng.gen::<(u8, u8, u8)>();
    let rand_point: Point = rng.gen();

    println!("rand_tuple: {:?}", rand_tuple);
    println!("rand_point: {:?}", rand_point);
}