// Rust program to calculate incentive

use std::io;

fn main() {
   let mut experience = String::new();
   let mut age = String::new();

   println!("\nIncentive Calculator");

   // input experience
   println!("\nThe employee is experienced (Enter true or false)");
   io::stdin().read_line(&mut experience).expect("Failed to read input");
   let experience:bool = experience.trim().parse().expect("Input is not boolean");

   // input age
   println!("Enter the employee's age");
   io::stdin().read_line(&mut age).expect("Failed to read input");
   let age:u8 = age.trim().parse().expect("Input not an integer");

   // incentive 
   if experience==true && age>=40 {
    println!("\nThe employee's annual incentive is ₦1_560_000.00");
   }
   else if experience==true && age>=30 && age<=39 {
    println!("\nThe employee's annual incentive is ₦1_480_000.00");
   }
   else if experience==true && age<=29 {
    println!("\nThe employee's annual incentive is ₦1_300_000.00");
   } 
   else if experience==false {
    println!("\nThe employee's annual incentive is ₦100_000.00");
    }
    else {
println!("\nNo annual incentive for such employee ");
    }

}
