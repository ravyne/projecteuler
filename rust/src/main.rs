mod p0001;

pub use crate::p0001::solutions;

fn main() {
    println!("Starting Project Euler...");
    println!("  Problem #1: Multiples of 3s or 5s...");
    println!("    Solution #1.1: simple -- {}", p0001::solutions::simple());

    println!("----------");
    println!("Complete.");
}
