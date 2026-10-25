use std::io::Write;
fn main() {
   let headings:[&str;4] = ["Student Name", "Matric.Number", "Department", "Level"];
   let std_name:[&str;5] = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
   let mat_num:[&str;5] = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
   let dept:[&str;5] = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
   let level:[&str;5] = ["300", "100", "200", "200", "100"];

   let heading = format!("{}, {}, {}, {} \n", headings[0],headings[1], headings[2], headings[3]);
   let student1 = format!("{}, {}, {}, {}\n", std_name[0], mat_num[0], dept[0], level[0]);
   let student2 = format!("{}, {}, {}, {}\n", std_name[1], mat_num[1], dept[1], level[1]);
   let student3 = format!("{}, {}, {}, {}\n", std_name[2], mat_num[2], dept[2], level[2]);
   let student4 = format!("{}, {}, {}, {}\n", std_name[3], mat_num[3], dept[3], level[3]);
   let student5 = format!("{}, {}, {}, {}\n", std_name[4], mat_num[4], dept[4], level[4]);

   let mut file = std::fs::File::create("PAU_SMIS.csv").expect("Failed to create");
   file.write_all("Student Management Information System\n".as_bytes()).expect("Failed to write");
   file.write_all(heading.as_bytes()).expect("Failed to write");
   file.write_all(student1.as_bytes()).expect("Failed to write");
   file.write_all(student2.as_bytes()).expect("Failed to write");
   file.write_all(student3.as_bytes()).expect("Failed to write");
   file.write_all(student4.as_bytes()).expect("Failed to write");
   file.write_all(student5.as_bytes()).expect("Failed to write");
   use std::io::Read;
   let mut file = std::fs::File::open("PAU_SMIS.csv").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   println!("{}", contents);
   
   println!("Data has been displayed");
   println!("File has been created and data has been written to it");
}