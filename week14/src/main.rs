use std::io::{self, Read};

// Define a struct to represent a user
struct User {
    role: String,
}

// Function to display the database structure based on user role
fn display_database_structure(user: &User) {
    match user.role.as_str() {
        "administrator" => println!("Displaying database structure..."),
        "project manager" => println!("Displaying structure of the project table..."),
        "employee" => println!("Displaying structure of the staff table..."),
        "customer" => println!("Calling the customer table..."),
        "vendor" => println!("Displaying data-plan table..."),
        _ => println!("Invalid role!"),
    }
}

fn main() {
    // Read user role from the console
    println!("Enter your role (administrator/project manager/employee/customer/vendor):");
    let mut role_input = String::new();
    io::stdin().read_line(&mut role_input).expect("Failed to read input");

    // Trim leading and trailing whitespaces
    let role = role_input.trim().to_lowercase();

    // Provide a default role if the input is empty
    let user = User {
        role: if role.is_empty() { "guest".to_string() } else { role },
    };

    // Display the corresponding database structure based on the user's role
    display_database_structure(&user);

    // Now, let's integrate the code to read a file and display its content
    match std::fs::File::open("globacom_dbase.sql") {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(err) = file.read_to_string(&mut contents) {
                eprintln!("Error reading file: {}", err);
            } else {
                println!("File contents:\n{}", contents);
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}