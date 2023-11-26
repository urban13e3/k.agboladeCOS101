use std::io;

fn main(){
    let mut count = 1;
    //Voting System for Student council
    loop{
        // Student name
        println!("Name of student:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let _student_name :String = input1.trim().parse().expect("Failed to read input");

        // Student Level
        println!("Student level:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let _student_level :u32 = input2.trim().parse().expect("Please enter a valid number");

        // Student CGPA
        println!("Student CGPA:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let _student_cgpa :f64 = input3.trim().parse().expect("Please enter a valid number");

        //Student Email
        println!("Student Email:");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let _student_email :String = input4.trim().parse().expect("Failed to read input");
        
        //Student department
        println!("Student Department:");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let _student_department :String = input5.trim().parse().expect("Failed to read input");

        //Student State Of Origin
        println!("State Of origin");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let _state_of_origin :String = input6.trim().parse().expect("Failed to read input");


        // Are you a Class Rep
        println!("Are you A Class Rep? (Type 'Yes' or 'No'):");
        let mut input7 = String::new();
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let _are_you_a_class_rep: String = input7.trim().to_lowercase(); 

        // Check Eligibility of Student (case-insensitive)
        if _student_level == 100 && _student_cgpa > 4.0 && _are_you_a_class_rep.eq_ignore_ascii_case("yes") {
            println!("You can vote");
            println!("Name: {}", _student_name);
            println!("Email: {}", _student_email);
            println!("Department: {}", _student_department);
            println!("State of Origin: {}", _state_of_origin);
        }else {
            println!("Sorry, you are not eligible to vote")
        }


        println!("Count:  {}", count);
        count +=1;

        if count >150 {
            break;
        }
    }
}