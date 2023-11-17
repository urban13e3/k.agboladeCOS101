use std::io;

fn main() {
    

// Patient Name
println!("Name of Patient :");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Not a valid string");
let _patient_name :String = input1.trim().parse().expect("Failed to read input");

//Patient Age
println!("Age :");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("Not a valid string");
let age: i32 = input2.trim().parse().expect("Not a valid number");

//Patient email address
println!("Email address :");
let mut input3 = String::new();
io::stdin().read_line(&mut input3).expect("Not a valid string");
let _email_address :String = input3.trim().parse().expect("Failed to read input");

//Patient phone number
println!("Phone Number :");
let mut input4 = String::new();
io::stdin().read_line(&mut input4).expect("Not a valid string");
let _phone_number :f64 = input4.trim().parse().expect("Not a valid number");

//Patient number of siblings
println!("Number of siblings :");
let mut input5 = String::new();
io::stdin().read_line(&mut input5).expect("Not a valid string");
let _number_of_siblings :i32 = input5.trim().parse().expect("Not a valid number");

//Patient number of children
println!("Number of children :");
let mut input6 = String::new();
io::stdin().read_line(&mut input6).expect("Not a valid string");
let _number_of_children :i32 = input6.trim().parse().expect("Not a valid number");

//Medical diagnosis
println!("Medical diagnosis :");
let mut input7 = String::new();
io::stdin().read_line(&mut input7).expect("Not a valid string");
let _medical_diagnosis:String = input7.trim().parse().expect("Failed to read input");

//Village of Residence
println!("Village of Residence :");
let mut input8 = String::new();
io::stdin().read_line(&mut input8).expect("Not a valid string");
let _village_of_residence:String = input8.trim().parse().expect("Failed to read input");

// Price for Alzheimer treatment
let alzheimer_treatment = 1200000.0;
// Check if a discount applies
if _number_of_children > 4 && _village_of_residence == "akapbom" && age > 50 {
    let discount = 0.2 * alzheimer_treatment;
    let discounted_total = alzheimer_treatment - discount;
    println!("20% Discount Applied! Discounted Total: N{}", discounted_total);
} else {
    println!("Total Order Amount: N{}", alzheimer_treatment);
}

// Price for arrhythmia
let arrhythmia_treatment = 550000.0;
// Check if a disount applies
if _number_of_siblings > 4 && _village_of_residence == "ngbauji" &&  age == 30 {
    let discount = 0.05 * arrhythmia_treatment;
    let discounted_total = arrhythmia_treatment - discount;
    println!("5% Discount Applied! Discounted Total: N{}", discounted_total);
} else {
    println!("Total Order Amount: N{}",arrhythmia_treatment);
}

//Price for Chronic Kidney Disease treatment
let chronic_kidney_disease = 1500000.0;
// Check if a discount applies
if _number_of_children > 4 && _number_of_siblings > 3 && _village_of_residence == "atabrikang" && age > 40 {
    let discount = 0.15 * chronic_kidney_disease;
    let discounted_total = chronic_kidney_disease - discount;
    println!("15% Discount Applied! Discounted Total: N{}", discounted_total);
} else {
    println!("Total Order Amount: N{}", chronic_kidney_disease);
}

 // Price for Diabetes treatment
let diabetes = 800000.0;
// Check if a discount applies
if _number_of_children > 4 && _village_of_residence == "okorobilom" && age > 28 && age < 45 {
    let discount = 0.1 * diabetes;
    let discounted_total = diabetes - discount;
    println!("10% Discount Applied! Discounted Total: N{}", discounted_total);
} else {
    println!("Total Order Amount: N{}", diabetes);
} 

//Price for Arthritis treatment
let arthritis_treatment = 450000.0;
 // Check if a discount applies
if _number_of_siblings > 5 && _number_of_children > 5 && _village_of_residence == "emeremen" && age > 58 {
    let discount = 0.1 * arthritis_treatment;
    let discounted_total = arthritis_treatment - discount;
    println!("10% Discount Applied! Discounted Total: N{}", discounted_total);
} else {
    println!("Total Order Amount: N{}", arthritis_treatment);
}
    // Function to get user input
    fn _get_user_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_uppercase().to_string()
    }

}
