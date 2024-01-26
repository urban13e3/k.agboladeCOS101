fn main() {
    let b:(i32, bool, f64) = (30, true, 4.9);
    print(b);
}

// pass the tuple as a parameter

fn print(x:(i32, bool, f64)) {
    println!("Inside print method");

    // assigns a tuple to distinct variables
    let (age, is_male, cgpa) = x;
    println!("Age is {}, isMale: {}, CGPA is {}", age,is_male,cgpa);
}