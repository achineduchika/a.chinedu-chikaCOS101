// Rust program to count numbers

use std::io;

fn main() {
   // input lower bound
   println!("\nPlease enter lower bound");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let lower_bound:i32 = input1.trim().parse().expect("Input not an integer");

   // input upper bound
   println!("\nPlease enter upper bound");
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("Failled to read input");
   let upper_bound:i32 = input2.trim().parse().expect("Input not an integer");
   
   for x in lower_bound..upper_bound{ // upper_bound is not inclusive

    println!("\nCount level is {}", x)
   }
}
