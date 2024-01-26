use std::io::Write;
use std::io;

fn main() {
    println!("**** NIGERIAN BREWRIRES PROJECT ****");

        let mut input1 = String::new();
    println!("How many lagers do you want to record: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let mut number_of_lager:u32 = input1.trim().parse().expect("Incorrect integer");

    let mut input2 = String::new();
    println!("How many stouts do you want to record: ");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let mut number_of_stout:u32 = input2.trim().parse().expect("Incorrect integer");

    let mut input3 = String::new();
    println!("How many non-alcoholic drinks do you want to record: ");
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let mut number_of_non_alcoholic:u32 = input3.trim().parse().expect("Incorrect integer");

    let mut lager: Vec<String> = Vec::new();
    let mut stout: Vec<String> = Vec::new();
    let mut non_alcoholic: Vec<String> = Vec::new();

    for _ in 0..number_of_lager {
        let mut input4 = String::new();
        println!("Lager name: ");
        io::stdin().read_line(&mut input4).expect("Invalid input");
        let larger_name = input4.trim().to_string();

        lager.push(larger_name); 
    }

    for _ in 0..number_of_stout {
        let mut input5 = String::new();
        println!("Stout name: ");
        io::stdin().read_line(&mut input5).expect("Invalid input");
        let stout_name = input5.trim().to_string();

        stout.push(stout_name); 
    }

    for _ in 0..number_of_non_alcoholic {
        let mut input6 = String::new();
        println!("Non-alcolic drink name: ");
        io::stdin().read_line(&mut input6).expect("Invalid input");
        let non_alcoholic_name = input6.trim().to_string();

        non_alcoholic.push(non_alcoholic_name); 
    }

    while number_of_lager > number_of_stout {
        stout.push(String::new());
        number_of_stout += 1;

        if number_of_lager == number_of_stout {
            break;
        } else {
            continue;
        }
    }

    while number_of_lager > number_of_non_alcoholic {
        non_alcoholic.push(String::new());
        number_of_non_alcoholic += 1;

        if number_of_lager == number_of_non_alcoholic {
            break;
        } else {
            continue;
        }
    }

    while number_of_stout > number_of_lager {
        lager.push(String::new());
        number_of_lager += 1;

        if number_of_stout == number_of_lager {
            break;
        } else {
            continue;
        }
    }

    while number_of_stout > number_of_non_alcoholic {
        non_alcoholic.push(String::new());
        number_of_non_alcoholic += 1;

        if number_of_stout == number_of_non_alcoholic {
            break;
        } else {
            continue;
        }
    }

    while number_of_non_alcoholic > number_of_stout {
        stout.push(String::new());
        number_of_stout += 1;

        if number_of_non_alcoholic == number_of_stout {
            break;
        } else {
            continue;
        }
    }

    while number_of_non_alcoholic > number_of_lager {
        lager.push(String::new());
        number_of_lager += 1;

        if number_of_non_alcoholic == number_of_lager {
            break;
        } else {
            continue;
        }
    }
    
    let mut file = std::fs::File::create("data.txt").expect("Create failed");
    file.write_all("--------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    file.write_all(
        format!("{:^19} {:^0} {:^19} {:^0} {:^25}\n", "LARGER", "|", "STOUT", "|", "NON-ALCOHOLIC")
            .as_bytes(),
    )
    .expect("Write failed");
    file.write_all("--------------------------------------------------------------------\n".as_bytes()).expect("Write failed");


    for i in 0..lager.len() {
        let first_item = &lager[i];
        let second_item = &stout[i];
        let third_item = &non_alcoholic[i];

        file.write_all(format!("{:^20}",first_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",second_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",third_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("--------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    }
}