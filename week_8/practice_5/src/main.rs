fn main(){
    let mut city :Vec<String> = Vec::new();
    println!("The city vector has element {}",city.len());

    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("Invalid input");
    for count in 0...city_num{
        let mut input2 = String::new();
    }
}