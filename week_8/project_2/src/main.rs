use std::io;

fn main() {
    let mut candidates_experience: Vec<i64> = Vec::new();
    let mut candidates_name: Vec<String> = Vec::new();

    println!("*** DEVELOPER SCOUTING PROGRAM ***");
    println!();

    let mut input1 = String::new();
    println!("How many people do you want to check: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let people_count: i32 = input1.trim().parse().expect("Invalid input");
    for _ in 0..people_count {
        let mut input2 = String::new();
        println!("Input your name as it appears on your passport: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let name = input2.trim().to_string();

        let mut input3 = String::new();
        println!("How many years of experience do you have: ");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let experience: i64 = input3.trim().parse().expect("Invalid input");

        println!("");
        println!("Next candidate!");
        candidates_experience.push(experience);
        candidates_name.push(name);
    }

    let max_value: i64 = *candidates_experience.iter().max().unwrap_or(&0);

    for (i, &experience) in candidates_experience.iter().enumerate() {
        if experience == max_value {
            println!("The candidate with the highest years of experience is: {}", candidates_name[i]);
            break;
        }
    }
}