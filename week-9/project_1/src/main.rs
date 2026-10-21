use std::io::Write;

fn main() {
   let header = "Lager, Stout, Non-Alcoholic";
   let column1 = "\n33 Export, Legend, Maltina";
   let column2 = "\nDesperados, Turbo King, Amstel Malta";
   let column3 = "\nGoldberg, Williams, Malta Gold";
   let column4 = "\nGuilder,,Fayrouz";
   let column5 = "\nHeineken,,";
   let column6 = "\nStar,,";

   let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("Failed to create");
   file.write_all("Welcome to the categories of drinks bottled by Nigerian Brewery Limited\n".as_bytes()).expect("Failed to write");
   file.write_all(header.as_bytes()).expect("Failed to write");
   file.write_all(column1.as_bytes()).expect("Failed to write");
   file.write_all(column2.as_bytes()).expect("Failed to write");
   file.write_all(column3.as_bytes()).expect("Failed to write");
   file.write_all(column4.as_bytes()).expect("Failed to write");
   file.write_all(column5.as_bytes()).expect("Failed to write");
   file.write_all(column6.as_bytes()).expect("Failed to write");

   println!("\nData has been written");
}
