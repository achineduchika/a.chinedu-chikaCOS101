// Rust program to loop count numbers upwards till it reaches 10

use std::io;

fn main() {
   
   println!("Please enter a number");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let mut num:i32 = input1.trim().parse().expect("Input not an integer");

   while num <10 {

    println!("\niNSIDE LOOP NUMBER VALUE IS {}", num);
    num+=1;
   }
   println!("\nOutside loop number value is {}", num);
}
