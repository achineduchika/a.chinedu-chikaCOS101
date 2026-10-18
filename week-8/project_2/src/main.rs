// Rust program for checking the candidate with the highest experience at an interview

use std::io;

fn main() {

    let mut candidates: Vec<(String,u8)> = Vec::new();

   println!("Welcome to the highest years of experince checker of candidates for The EY Interview");
   println!("\nThis programme helps to narrow down the candidate with the highest number of interviews");
   println!("\nPlease Enter the number of candidates interviewed: ");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");
   let input:usize = input.trim().parse().expect("Invalid input");

   for i in 0..input {
    println!("Please Enter the name of Candidate {}", i + 1);
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim().to_string();
    println!("Kindly enter {}'s year(s) of programming language: ", name);
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let years:u8 = years.trim().parse().expect("Invalid input");

    candidates.push((name,years));
   }

   if candidates.len() == 0 {
    println!("There were no candidates interviewed today");
   }

   let mut highest_experince = &candidates[0];
   for candidates in &candidates {
    if candidates.1 > highest_experince.1 {
        highest_experince = candidates;
    }
   }
   println!("The candidate with the highest years of programming experience is {} with {} years of experience.", highest_experince.0, highest_experince.1);
}
