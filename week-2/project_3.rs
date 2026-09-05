fn main() {
   let p:f64 = 210_000.00;
   let r:f64 = 5.0;
   let t:f64 = 3.0;

   // depreciated value
   let d = p * ( 1.0 - (r/100.00)).powf(t);
   println!("Depreciated Value is ₦{:.2}", d);

}