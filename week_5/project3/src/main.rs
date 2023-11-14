use std::io;

fn main() {
    // Display Menu For Food Items and prices
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    // Input the type of food ordered
    let _food_type = get_user_input("Enter the type of food (P/F/A/E/W):");

    // Input the quantity of food
    println!("Amount for P (Pounded Yam/Edinkaiko Soup)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let P: f64 = input1.trim().parse().expect("Not a valid number");

    println!("Amount for F (Fried Rice & Chicken)");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let F: f64 = input2.trim().parse().expect("Not a valid number");

    println!("Amount for A (Amala & Ewedu Soup)");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let A: f64 = input3.trim().parse().expect("Not a valid number");

    println!("Amount for E (Eba & Egusi Soup)");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let E: f64 = input4.trim().parse().expect("Not a valid number");

    println!("Amount for W (White Rice & Stew)");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let W: f64 = input5.trim().parse().expect("Not a valid number");

    // Calculate total charges for each food item
    let total_charges_P = P * 3200.0;
    let total_charges_F = F * 3000.0;
    let total_charges_A = A * 2500.0;
    let total_charges_E = E * 2000.0;
    let total_charges_W = W * 2500.0;

    // Calculate the total charges for the user's order
    let order_total = total_charges_P + total_charges_F + total_charges_A + total_charges_E + total_charges_W;

    // Display the total charges for the user's order
    println!("Total Charges For User Order: N{}", order_total);

    // Check if a discount applies
    if order_total > 10000.0 {
        let discount = 0.05 * order_total;
        let discounted_total = order_total - discount;
        println!("5% Discount Applied! Discounted Total: N{}", discounted_total);
    } else {
        println!("Total Order Amount: N{}", order_total);
    }
}

// Function to get user input
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_uppercase().to_string()
}
