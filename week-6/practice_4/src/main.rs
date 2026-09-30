fn main() {
   let fullname = "Chinedu-Chika Angel Mmesoma";
   let department = "Software Engineering";
   let uni = "Pan-Atlantic University";

   let mut school = "Scool of Science".to_string();
   //push string
   school.push_str(" and Technology");

   println!("My name is {}", fullname);
   //check length
   println!("The length of my fullname is: {}",fullname.len());
   println!("I am a student of {} Department", department);
   println!("{}", school );
   println!("{}", uni);
}
