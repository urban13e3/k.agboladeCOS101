use std::io;

fn main(){
    let _count =1;
    loop{

    //Choose an equation
    println!("Select an equation:");
    println!("1. Area of Trpezium:");
    println!("2. Area of Rhombus:");
    println!("3. Area of Parallelogram:");
    println!("4. Area of Cube:");
    println!("5. Volume of Cylinder:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    if choice ==1{
        println!("Value of height: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let _height :f64 = input1.trim().parse().expect("Failed to read input");

        println!("Value of base 1:");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let _base_1 :f64 = input4.trim().parse().expect("Failed to read input");

        println!("Value of base 2:");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let _base_2 :f64 = input5.trim().parse().expect("Failed to read input");

        println!("Area of Trapezium:");
        let _area_of_trapezium = _height / 2.0 * (_base_1 + _base_2);
        println!("{}" ,_area_of_trapezium);
    }

    if choice==2{
        println!("Value of diagonal 1:");
        let mut input2 =  String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let _diagonal_1 :f64 = input2.trim().parse().expect("Failed to read input");     

        println!("Value of diagonal 2:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let _diagonal_2 :f64 = input3.trim().parse().expect("Failed to read input");
    
        println!("Area of Rhombus:");
        let _area_of_rhombus :f64 = 0.5 * _diagonal_1 * _diagonal_2;
        println!(" {}", _area_of_rhombus);
    }

    if choice == 3{
        println!("Value of base:");
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let _base :f64 = input8.trim().parse().expect("Failed to read input");

        println!("Value of altitude: ");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("failed to read input");
        let _altitude :f64 = input6.trim().parse().expect("Failed to read input");

        println!("Area of Parellogram:");
        let _area_of_parallegram :f64 = _base * _altitude;
        println!("{}", _area_of_parallegram);
    }

    if choice ==4{
        println!("Value of length: ");
        let mut input7 = String::new();
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let _length :f64 = input7.trim().parse().expect("Failed to read input");

        println!("Area of cube:");
        let _area_of_cube :f64 =6.0 * (_length * _length);
        println!(" {}", _area_of_cube);
    }

    if choice ==5{
        println!("Value of height: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let _height :f64 = input1.trim().parse().expect("Failed to read input");

        println!("Value of radius: ");
        let mut input9 = String::new();
        io::stdin().read_line(&mut input9).expect("Failed to read input");
        let _radius :f64 = input9.trim().parse().expect("Failed to read input");

        println!("Volume of Cylinder:");
        let _volume_of_cylinder :f64 = 3.142 * _radius * _height;
        println!(" {}", _volume_of_cylinder);

    }

}
}