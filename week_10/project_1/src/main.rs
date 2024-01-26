// Define a struct for each laptop brand
struct Laptop {
    brand: String,
    price: u64,
    quantity: u32,
}

// Implement a method to calculate the total cost for a specific laptop brand
impl Laptop {
    fn calculate_cost(&self) -> u64 {
        self.price * u64::from(self.quantity)
    }
}

fn main() {
    // Create instances of each laptop brand
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        price: 650000,
        quantity: 3,
    };

    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        price: 755000,
        quantity: 3,
    };

    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
        quantity: 3,
    };

    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        price: 850000,
        quantity: 3,
    };

    // Calculate the total cost for each laptop brand
    let total_cost_hp = hp_laptop.calculate_cost();
    let total_cost_ibm = ibm_laptop.calculate_cost();
    let total_cost_toshiba = toshiba_laptop.calculate_cost();
    let total_cost_dell = dell_laptop.calculate_cost();

    // Calculate the overall total cost
    let overall_total_cost = total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    // Display the results
    println!("Total cost for HP laptops: {}", total_cost_hp);
    println!("Total cost for IBM laptops: {}", total_cost_ibm);
    println!("Total cost for Toshiba laptops: {}", total_cost_toshiba);
    println!("Total cost for Dell laptops: {}", total_cost_dell);
    println!("Overall total cost: {}", overall_total_cost);
}