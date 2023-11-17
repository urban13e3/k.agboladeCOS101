use std::io;

fn main() {
    // Input for the value of n
    println!("Enter the value of n:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    // Display the multiplication table vertically
    for x in 1..=1 {
        for y in 1..=n {
            // Calculate and display the product
            let result = x * y;
            println!("{} x {} = {}", x, y, result);
        }
    }
}
