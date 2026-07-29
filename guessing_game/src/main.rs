use rand::{Rng, RngExt};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("🎯 Welcome to the Guessing Game!");
    println!("I am thinking of a number between 1 and 100. Can you guess it?");

    // Generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);
}
