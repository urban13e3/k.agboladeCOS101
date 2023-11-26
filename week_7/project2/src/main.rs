use std::io;

fn main(){
    let mut _count = 1;
    loop{
    //Name of client
    println!("Name of client:");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _name_of_client :String = input1.trim().parse().expect("Failed to read input");

    //Do you have a sibling?
    println!("Do you have a sibling? (Type 'Yes' or 'No')");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _do_you_have_a_sibling :String = input2.trim().to_lowercase(); 

    if _do_you_have_a_sibling.eq_ignore_ascii_case("no") {
        println!("Client does not have a sibling. Exiting the program.");
        break; // Break out of the loop
    }


    if _do_you_have_a_sibling.eq_ignore_ascii_case("yes"){
        
        //How old is your sibling?
        println!("How old is your sibling?");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let _age_of_sibling :u32 = input3.trim().parse().expect("Failed to read input");

        if _age_of_sibling > 18 {

            //Is your sibling married?
            println!("Is your sibling married? (Type 'Yes' or 'No')");
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read input");
            let _is_sibling_married: String = input4.trim().to_lowercase();
        
        if _is_sibling_married.eq_ignore_ascii_case("yes"){
           
            println!("How many offspring does your sibling have?");
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Failed to read input");
            let _offspring_count: u32 = input5.trim().parse().expect("Please enter a valid number");

            println!("In which city does your sibling's family live?");
            let mut input6 = String::new();
            io::stdin().read_line(&mut input6).expect("Failed to read input");
            let _family_city: String = input6.trim().parse().expect("Failed to read input");
        }
    }
    } else{

            println!("Is your sibling a student or a worker?");
            let mut input7 = String::new();
            io::stdin().read_line(&mut input7).expect("Failed to read input");
            let _sibling_occupation: String = input7.trim().to_lowercase();

                if _sibling_occupation.eq_ignore_ascii_case("student") {
                    println!("Which university does your sibling attend?");
                    let mut input8 = String::new();
                    io::stdin().read_line(&mut input8).expect("Failed to read input");
                    let _sibling_university: String = input8.trim().parse().expect("Failed to read input");

                    println!("What is your sibling's course of study?");
                    let mut input9 = String::new();
                    io::stdin().read_line(&mut input9).expect("Failed to read input");
                    let _sibling_course: String = input9.trim().parse().expect("Failed to read input");
                }
        }eprintln!("Does your sibling have a WAEC result? (Type 'Yes' or 'No')");
            let mut input10 = String::new();
            io::stdin().read_line(&mut input10).expect("Failed to read input");
            let _waec_status: String = input10.trim().to_lowercase();

            if _waec_status.eq_ignore_ascii_case("yes") {
                println!("Which secondary school did your sibling attend?");
                let mut input11 = String::new();
                io::stdin().read_line(&mut input11).expect("Failed to read input");
                let _secondary_school: String = input11.trim().parse().expect("Failed to read input");
            } else {
                // If no, input the current class level
                println!("In which class is your sibling currently?");
                let mut input12 = String::new();
                io::stdin().read_line(&mut input12).expect("Failed to read input");
                let _current_class: String = input12.trim().to_string();
            }

        println!("Count: {}", _count);  
        _count += 1;
    }
}


    
