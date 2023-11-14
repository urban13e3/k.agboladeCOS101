use std::io;

fn main(){
    println!("Enter the coefficient a:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a:f64 = input_a.trim().parse().expect("Invalid input. Enter a number.");

    println!("Enter the coefficient b:");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b:f64 = input_b.trim().parse().expect("Invalid input. Enter a number.");

    println!("Enter the coefficient c:");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c:f64 = input_c.trim().parse().expect("Invalid input. Enter a number.");


    //Calculate discriminant
    let discriminant = b.pow(2) - 4.0 * a * c;

    //Determine roots based on discriminant
    if discriminant > 0.0{
        //Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Root 1: {}", root1);
        println!("Root 2: {}", root2);
    }else if discriminant == 0.0 {
        // Exactly one real root
        let root = -b / (2.0 * a);
        println!("Single Root: {}", root);
    } else {
        // No real roots (complex roots)
        let real_part = -b / (2.0 * a);
        let immaginary_part = (discriminant.abs()).sqrt() / (2.0 * a);
        println!("Complex root 1: {} + {}i", real_part, immaginary_part);
        println!("Complex Root 2: {} - {}i", real_part, immaginary_part);
    }

}