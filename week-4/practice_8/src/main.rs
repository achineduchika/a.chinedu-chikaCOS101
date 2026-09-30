// Rust program to count till 15

fn main() {
   
   // while true

   let mut x = 0;
   loop{
    x+=1;
    println!("x={}", x);

    if x==15{
        break;
    }
   }
}