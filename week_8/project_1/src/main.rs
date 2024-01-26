use std::io;

fn main() {
    let mut candidates: Vec<(String, i64, String)> = Vec::new();

    println!("*** PUBLIC SERVICE APS LEVEL CHECKER ***");
    println!("*** YOUR APS LEVEL IS GIVEN TO YOU BASED ON YOUR POSITION IN: ");
    println!("*** OFFICE ADMISTRATION, ACADEMICS, LAW OR TEACHING");

    let mut input1 = String::new();
    println!("How many people do you want to check: ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let people_count:i32 =  input1.trim().parse().expect("Invalid input");

    
    for _ in 0..people_count {
        let mut input2 = String::new();
        println!("Input your name as it appears on your passport: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let name = input2.trim().to_string();

        let mut input3 = String::new();
        println!("How many years of experience do you have: ");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let experience: i64 = input3.trim().parse().expect("Invalid input");

        let mut input4 = String::new();
        println!("What position do you hold in OFFICE ADMISTRATION, ACADEMICS, LAW OR TEACHING");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let position = input4.trim().to_string();

        candidates.push((name.clone(), experience, position.clone()));

        
        if (experience == 1 || experience == 2) && (position.to_lowercase() == "intern" || position.to_lowercase() == "paralegal" || position.to_lowercase() == "placement" || position.to_lowercase() == "none") {
            println!("Congratulations! \nYou are eligible for the APS 1-2 level");
        } else if (3..=5).contains(&experience) && (position.to_lowercase() == "administrator" || position.to_lowercase() == "research assistant" || position.to_lowercase() == "junior associate" || position.to_lowercase() == "classroom teacher") {
            println!("Congratulations! \nYou are eligible for the APS 3-5 level");
        } else if (5..=8).contains(&experience) && (position.to_lowercase() == "senior administrator" || position.to_lowercase() == "phd candidate" || position.to_lowercase() == "associate" || position.to_lowercase() == "senior teacher") {
            println!("Congratulations! \nYou are eligible for the APS 5-8 level");
        } else if (8..=10).contains(&experience) && (position.to_lowercase() == "office manager" || position.to_lowercase() == "post-doc researcher" || position.to_lowercase() == "senior associate 1-2" || position.to_lowercase() == "leading teacher") {
            println!("Congratulations! \nYou are eligible for the EL1 8-10 level");
        } else if (10..=13).contains(&experience) && (position.to_lowercase() == "director" || position.to_lowercase() == "senior lecturer" || position.to_lowercase() == "senior associate 3-4" || position.to_lowercase() == "deputy principal") {
            println!("Congratulations! \nYou are eligible for the EL2 10-13 level");
        } else if experience >= 13 && (position.to_lowercase() == "ceo" || position.to_lowercase() == "dean" || position.to_lowercase() == "partner" || position.to_lowercase() == "principal") {
            println!("Congratulations! \nYou are eligible for the APS SES level");
        } else {
            println!("Sorry, but you aren't eligible.");
        }
    }

    for candidate in &candidates {
        println!("Your name is: {}\nYou have {} years of experience\nYour position is: {}\n", candidate.0, candidate.1, candidate.2);
    }
}