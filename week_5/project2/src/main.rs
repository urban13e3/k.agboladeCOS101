use std::io;

fn main() {
    // Input employee's experience and age
    println!("Enter employee's experience (experienced or inexperienced):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim();

    println!("Enter employee's age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Invalid input. Enter a number");

    // Determine annual incentive based on experience and age
    let incentive = match experience {
        "experienced" => {
            if age >= 40 {
                1560000
            } else if age >= 30 && age < 40 {
                1480000
            } else if age < 28 {
                1300000
            } else {
                0 // This case should not happen, but handling it to avoid uninitialized variable warning
            }
        }
        "inexperienced" => 100000,
        _ => {
            println!("Invalid input for experience. Please enter 'experienced' or 'inexperienced'.");
            0 // Default to 0 incentive for invalid input
        }
    };

    // Print the annual incentive
    if incentive > 0 {
        println!("Annual Incentive: N{}", incentive);
    } else {
        println!("Invalid input. Incentive could not be determined.");
    }
}
