// Rust program for a restaurant menu


use std::io;

fn main() {
    println!("The Restaurant Menu");
    println!("P - Pounded Yam / Edinkaiko Soup - ₦3200");
    println!("F - Fried Rice & Chicken - ₦3000");
    println!("A - Amala & Ewedu Soup - ₦2500");
    println!("E - Eba & Egusi Soup - ₦2000");
    println!("W - White Rice & Stew - ₦2500");
    println!("Type 'Done' to finish ordering.");

    let mut total_cost: f32 = 0.0;

    loop {
        println!("\nPlease enter the food type of your choice and ensure to type Done to finish:");
        // Input food type
        let mut food_type = String::new();
        let _ = io::stdin().read_line(&mut food_type);
        let food_type = food_type.trim();
        if food_type == "Done" {
            break; 
        }

        // Determine price
        let price: f32 = if food_type == "P" {
            3200.0
        } else if food_type == "F" {
            3000.0
        } else if food_type == "A" {
            2500.0
        } else if food_type == "E" {
            2000.0
        } else if food_type == "W" {
            2500.0
        } else {
            println!("Please enter a valid food type.");
            continue; 
        };

        println!("Nice choice! Now enter the quantity:");
        
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Not a valid string");
         let quantity: f32 = quantity.trim().parse().expect("Not a valid integer");

        // Calculate the total cost
        let cost = price * quantity;
        total_cost += cost;

    }

        println!("\n--- Final Bill ---");
    if total_cost > 10000.0 {
        let discount = total_cost * 0.05;
        let final_cost = total_cost - discount;
        
        
        println!("A discount of ₦{:.2} has been applied to your total", discount);
        
        println!("Your Total cost is now ₦{:.2}", final_cost);
    } else {
        println!("Your Total cost is ₦{:.2}", total_cost);
    }
    
    println!("Thank you for dining with us and do come back again!");
}
