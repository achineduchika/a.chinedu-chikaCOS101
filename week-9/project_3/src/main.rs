use std::io::Write;

fn main() {
   let headings = ["Commissioner", "Ministry", "Zone"];
   let commissioner = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Clistus", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
   let ministry = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
   let zone = ["South West", "North East", "South South", "South West", "South East"];

   let heading = format!("{}, {}, {}\n",headings[0], headings[1],headings[2]);

   let mut file = std::fs::File::create("The_EFCC_Convicts.csv").expect("Failed to create");
   file.write_all("Ministers convicted by The EFCC\n".as_bytes()).expect("Failed to write");
   file.write_all(heading.as_bytes()).expect("Failed to write");
   for i in 0..commissioner.len() {
    let rows = format!("{}, {}, {}\n", commissioner[i], ministry[i], zone[i]);  
   file.write_all(rows.as_bytes()).expect("Failed to write");
}
   println!("File has been created and the merged data has been written to it");
}
