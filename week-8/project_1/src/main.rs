// Rust program for an APS level checker

use std::io;


fn main() {
   println!("\nPublic Service APS Level Checker for The Federal Government of Nigeria");
   println!("\nThis app helps you to check for your APS level or your profession as a public servant");
   println!("\nNow it's your choice to choose how to use the Checker, through:\n1. Searching by profession. \n2. Searching by APS level");
   println!("Kindly choose by typing 1 or 2");
   let mut choice = String::new();
   io::stdin().read_line(&mut choice).expect("Failed to read input");
   let choice:u8 = choice.trim().parse().expect("Invalid input");


   if choice == 1 {
    choice_1();
   } else if choice == 2 {
    choice_2();
   } else {
    println!("Please enter a suitable option");
   }
}

fn choice_1() {
    println!("You can begin by selecting your profession category from below:\n1. Office Admin\n2. Academic\n3. Lawyer\n4. Teacher");
     let mut category = String::new();
   io::stdin().read_line(&mut category).expect("Failed to read input");
   let category:u8 = category.trim().parse().expect("Invalid input");

     let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 6-10", "EL2 10-13", "SES"];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    if category == 1 {
        println!("The Office Admin professions are: ");
        for i in 0..office_admin.len() {
            println!("{}", office_admin[i]);
        }
    } else if category == 2 {
        println!("The Academic professions are: ");
        for i in 0..academic.len() {
            println!("{}", academic[i]);
        }
    } else if category == 3 {
        println!("The Lawyer professions are: ");
        for i in 0..lawyer.len() {
            println!("{}", lawyer[i]);
        }
    } else if category == 4 {
        println!("The Teacher professions are: ");
        for i in 0..teacher.len() {
            println!("{}", teacher[i]);
        }
    } else {
        println!("Please enter a valid profession category either 1 or 2 or 3 or 4");
        return;
    }

        println!("Kindly enter your current profession");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim();

    if category == 1 {
        for i in 0..office_admin.len() {
            if office_admin[i] == profession{
                println!("Since your profession is {}, your APS level is {}.", office_admin[i], levels[i]);
                return;
            }
        }
    } else if category == 2 {
        for i in 0..academic.len() {
            if academic[i] == profession {
                println!("Since your profession is {}, your APS level is {}.", academic[i], levels[i]);
                return;

            }
        }
    } else if category == 3 {
        for i in 0..lawyer.len() {
            if lawyer[i] == profession {
                println!("Since your profession is {}, your APS level is {}.", lawyer[i], levels[i]);
                return;
            }
        }
    } else if category == 4 {
        for i in 0..teacher.len() {
            if teacher[i] == profession {
                println!("Since your profession is {}, your APS level is {}.", teacher[i], levels[i]);
                return;
            }
        }
    } else {
        println!("Please enter a suiltable profession from the categories");
        return;
    }  println!("Please enter a suiltable profession from the categories");
}

fn choice_2() {
println!("You can begin by selecting your profession category from below:\n1. Office Admin\n2. Academic\n3. Lawyer\n4. Teacher");
    let mut category = String::new();
   io::stdin().read_line(&mut category).expect("Failed to read input");
   let category:u8 = category.trim().parse().expect("Invalid input");

   let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 6-10", "EL2 10-13", "SES"];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

  println!("The APS level are: APS 1-2, APS 3-5, APS 5-8, EL1 8-10, EL2 10-13, SES");
  println!("Please enter your current APS level:");
  let mut aps = String::new();
  io::stdin().read_line(&mut aps).expect("Failed to read input");
  let aps = aps.trim();

    for i in 0..levels.len() {
        if levels[i] == aps {
            if category == 1 {
                println!("Since your APS level is {} then you are a/an {}", levels[i], office_admin[i]);
                return;
            } else if category == 2 {
                println!("Since your APS level is {} then you are a/an {}", levels[i], academic[i]);
                return;
            } else if category == 3 {
                println!("Since your APS level is {} then you are a/an {}", levels[i], lawyer[i]);
                return;
            } else if category == 4 {
                println!("Since your APS level is {} then you are a/an {}", levels[i], teacher[i]);
                return;
            } else {
                println!("Please enter a valid category");
                return;
            }
        }
    }  println!("Please enter a valid APS level");
}