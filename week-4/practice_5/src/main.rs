// Rust program to read the height of a person
// and then print if the person is tall, dwarf,
// or average heighted person

use std::io;

fn main() {
   let mut input = String::new();

   // input height
   println!("\nPlease Enter Your Height (in centimetres): ");
   io::stdin().read_line(&mut input).expect("Not a valid string");
   let height:f32 = input.trim().parse().expect("Not a valid number");

   if height >=150.00 && height <= 170.00{
      println!("\nYou are an average heighted person");
   } 
   else if height > 170.00 && height <= 195.00{
    println!("\nYou are tall");
   } 
   else  if height < 150.00 && height > 100.00{
    println!("\nYou are a dwarf");
   } 
   else {
    println!("\nAbnormal height");
   }
   
}
