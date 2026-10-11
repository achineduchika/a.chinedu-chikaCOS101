// Rust program for a shape calculator

use std::io;

fn trap_area(){
    println!("\nGood you have chosen Trapezium, now enter it's height: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
   let height:f64 = height.trim().parse().expect("Invalid integer");
   println!("Then enter the first base: ");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read input");
   let base1:f64 = base1.trim().parse().expect("Invalid integer");
   println!("And finally enter the second base: ");
   let mut base2 = String::new();
   io::stdin().read_line(&mut base2).expect("Failed to read input");
   let base2:f64 = base2.trim().parse().expect("Invalid integer");
   // calculate the area
   let trap_area:f64 = (height / 2.0) * (base1 + base2);
   println!("\nThe area of the Trapezium is: {}", trap_area);
}

fn rhom_area(){
   println!("\nGood you have chosen Rhombus, now enter it's fist diagonal: ");
   let mut d1 = String::new();
   io::stdin().read_line(&mut d1).expect("Failed to read input");
   let d1:f64 = d1.trim().parse().expect("Invalid integer");
   println!("Then enter it's second diagonal: ", );
   let mut d2 = String::new();
   io::stdin().read_line(&mut d2).expect("Failed to read input");
   let d2:f64 = d2.trim().parse().expect("Invalid integer");
   // calculate the area
   let rhom_area:f64 = 0.5 * d1 * d2;
   println!("\nThe area of the Rhombus is: {}", rhom_area);
}

fn para_area(){
   println!("\nGood you have chosen Parallelogram, now enter it's base: ");
   let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed to read input");
   let base:f64 = base.trim().parse().expect("Invalid integer"); 
   println!("Then enter it's altitude: ");
   let mut alt = String::new();
   io::stdin().read_line(&mut alt).expect("Failed to read input");
   let alt:f64 = alt.trim().parse().expect("Invalid integer");
   // calculate area
   let para_area:f64 = base * alt;
   println!("\nThe area of the Parallelogram is: {}", para_area);
}

fn cube_s_area(){
   println!("\nGood you have chosen Cube, now enter it's side: ");
   let mut side = String::new();
    io::stdin().read_line(&mut side).expect("Failed to read input");
   let side:f64 = side.trim().parse().expect("Invalid integer"); 
         // calculate surface area
   let cube_s_area:f64 = 6.0 * side * side;
   println!("\nThe surface area of the Cube is: {}", cube_s_area);
}

fn cyl_vol(){
   println!("\nGood you have chosen Cylinder, now enter it's radius: ");
   let mut radius = String::new();
   io::stdin().read_line(&mut radius).expect("Failed to read input");
   let radius:f64 = radius.trim().parse().expect("Invalid integer"); 
   println!("Then enter it's height: ");
   let mut height1 = String::new();
   io::stdin().read_line(&mut height1).expect("Failed to read input");
   let height1:f64 = height1.trim().parse().expect("Invalid integer");
   // get pi
   let a:f64 = 22.0;
   let b:f64 = 7.0;
   let pi:f64 = 22.0 / 7.0;
   // calculate area
   let cyl_vol:f64 = pi * radius * radius * height1;
   println!("\nThe volume of the Cylinder is: {}", cyl_vol);
}

fn main() {
   println!("\nWelcome Dear Mathematician");
   println!("\nThe Shape Calculator");
   println!("1. Tapezium (Area)");
   println!("2. Rhombu (Area)");
   println!("3. Parallelogram (Area)");
   println!("Cube (Surface Area)");
   println!("Cylinder (Volume)");

   loop{
   println!("\nEnter the corresponding number (1, 2, 3, 4 or 5) for the shape you want to calculate on. Enter 0 when done");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let shape_choice:u8 = input1.trim().parse().expect("Invalid integer");
   if shape_choice == 1 {
   trap_area();
   } else if shape_choice == 2 {
   rhom_area();
   } else if shape_choice == 3 {
   para_area();
   } else if shape_choice == 4 {
    cube_s_area();
   } else if shape_choice == 5 {
    cyl_vol();
   } else if shape_choice == 0 {
   println!("\nThankyou for using The Shape Calculator");
   break;
   } else {
    println!("Enter a valid shape choice.");
   }
   
}

}