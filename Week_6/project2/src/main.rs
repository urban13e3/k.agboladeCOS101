use std::io;

//RPIS (Researchers Publication Incentive System)
fn main (){
    let mut count = 1;
loop {
    //Researcher Name
    println!("Name of Research;");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _name_of_research :String = input1.trim().parse().expect("Failed to read input");

    //Research Number Of Papers Published
    println!("Number Of Paper Published:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _number_of_research :u32 = input2.trim().parse().expect("Please enter a valid number");

    //Incentive: Between 3-5 published papers
    if _number_of_research >=3 && _number_of_research <=5 {
        let _incentive_obtained = 500000.0;
        println!("Incentive Obtained: N{}", _incentive_obtained);
        println!("Name of Researcher: {}", _name_of_research);
    }

    //Incentive: Over 5 but Less than 10 published papers
    if _number_of_research >5 && _number_of_research <=10{
        let _incentive_obtained = 800000.0;
        println!("Incentive Obtained: N{}", _incentive_obtained);
        println!("Name of Researcher: {}", _name_of_research);
    }

    //Incentive: Over 10 published papers
    if _number_of_research >10{
        let _incentive_obtained = 1000000.0;
        println!("Incentive Obtained: N{}", _incentive_obtained);
        println!("Name of Researcher: {}", _name_of_research);
    }



    println!("Count: {}", count);
    count +=1;

    if count >=500 {
        break;    
    } 
    
}
println!("system  has reached max limit ");
}