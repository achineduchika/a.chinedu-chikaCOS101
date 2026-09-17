// Rust program to determine age pass

use std::io;

fn main() {
   
   let mut input1 = String::new();
   let mut input2 = String::new();

   println!("\nParty Gate");

   // input name
   println!("\nPlease enter your Full name");
   io::stdin().read_line(&mut input1).expect("Not a valid string");

   // input age
   println!("Please enter you age: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let age:u8 = input2.trim().parse().expect("Not a valid integer");

   if age >= 21 {
      println!("\nWelcome to the party {}!", input1);
  } else {
    println!("\nOops, you are not of age to enter the party {}", input1);
  }
   }