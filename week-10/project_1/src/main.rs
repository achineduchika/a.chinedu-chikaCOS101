struct Laptop {
    brand:String,
    quantity:u32,
    price:u32
}
impl Laptop {
    fn total_cost(&self)->u32{
     self.quantity * self.price
    }
}
fn main() {
    let l1 = Laptop {
        brand:String::from("HP"),
        quantity:3,
        price:650_000
    };
    let l2 = Laptop {
        brand:String::from("IBM"),
        quantity:3,
        price:755_000,
    };
    let l3 = Laptop {
        brand:String::from("Toshiba"),
        quantity:3,
        price:550_000
    };
    let l4 = Laptop {
        brand:String::from("Dell"),
        quantity:3,
        price:850_000
    };
    let total_l1 = l1.total_cost();
    let total_l2 = l2.total_cost();
    let total_l3 = l3.total_cost();
    let total_l4 = l4.total_cost();

    let bill = total_l1 + total_l2 + total_l3 + total_l4;

    println!("Dear customer, after choosing to purchase three laptops each your total cost is\n₦{}", bill);
}
