// Rust program to calculate the roots of a quadratic equation

use std::io;

fn main() {

   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

// input first number
   println!("\nEnter the first coefficient");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a:f32 = input1.trim().parse().expect("Input not a number");

   // input second number
   println!("\nEnter the second coefficient");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b:f32 = input2.trim().parse().expect("Input not a number");

   // input third number
   println!("\nEnter the third coefficient");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let c:f32 = input3.trim().parse().expect("Input not a number");

   // discriminant
   let d:f32 = b * b - (4.0 * a * c);

   if d > 0.0 {
    let root1:f32 = (- b + d.sqrt()) / (2.0 * a);
    let root2:f32 = (- b - d.sqrt()) / (2.0 * a);
    println!("There are two distinct roots: {} and {}", root1, root2 );
    }
    else if d==0.0 {
        let root:f32 = (- b) / (2.0 * a);
        println!("There is exactly one real root: {}", root);
    }
    else {
        println!("There are no real roots");
    }
}
