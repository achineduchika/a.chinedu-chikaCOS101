// Rust program to calculate the area of a triangle with its givrn three side

use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

  // input first side
   println!("Enter first side of the triangle: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let a:f32 = input1.trim().parse().expect("Not a valid number");

   // input second side
   println!("Enter second side of the triangle: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let b:f32 = input2.trim().parse().expect("Not a valid number");

   // input third side
   println!("Enter third side of the triangle: ");
   io::stdin().read_line(&mut input3).expect("Not a valid sting");
   let c:f32 = input3.trim().parse().expect("Not a valid number");

   // area of the three sides
   let s:f32 = (a + b + c) / 2.0;
   let mut area:f32 = s * (s - a) * (s - b) * (s - c);
   area = area.sqrt();

   println!("\nArea of a triangle {}", area);
}
