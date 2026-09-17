// Rust program to calculate the area of a triangle with it given base and height

use std::io;

fn main() {
   
   let mut input1 = String::new();
   let mut input2 = String::new();

   // input base
   println!("Enter the base: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let base:f32 = input1.trim().parse().expect("Not a valid number");

   // input height
   println!("Enter the height: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let height:f32 = input2.trim().parse().expect("Not a valid number");

   // area
   if base > 0.0 && height > 0.0 {
      let area:f32 = (base * height) / 2.0;
      println!("Area of the triangle: {}", area);
  }
}  