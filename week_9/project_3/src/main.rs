fn main() {
    let n = vec![
        "Aigbogun Alamaba Dauda",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let m = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];

    let g = vec!["South West", "North East", "South South", "South West", "South East"];

    println!("{:<30} | {:<20} | {:<15}", "Name", "Ministry", "Region");
    println!("{:-<30} | {:-<20} | {:-<15}", "", "", ""); // Separator line

    for i in 0..n.len() {
        println!("{:<30} | {:<20} | {:<15}", n[i], m[i], g[i]);
    }
}use std::io::Write;
use std::io;

fn main() {
    println!("**** EFCC PROJECT ****");

   
    let mut input1 = String::new();
    println!("How many commissioner do you want to record: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let number_of_commissioners:u32 = input1.trim().parse().expect("Incorrect integer");

    let mut name: Vec<String> = Vec::new();
    let mut ministry: Vec<String> = Vec::new();
    let mut geopolitical_area: Vec<String> = Vec::new();

    for _ in 0..number_of_commissioners {
        let mut input2 = String::new();
        println!("Name of Commissioner: ");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        let commissioner_name = input2.trim().to_string();
        name.push(commissioner_name);

        let mut input3 = String::new();
        println!("Ministry: ");
        io::stdin().read_line(&mut input3).expect("Invalid input");
        let ministry_name = input3.trim().to_string();
        ministry.push(ministry_name);

        let mut input4 = String::new();
        println!("Geopolitical Area: ");
        io::stdin().read_line(&mut input4).expect("Invalid input");
        let geo_area = input4.trim().to_string();
        geopolitical_area.push(geo_area);

    }

    let mut file = std::fs::File::create("data.txt").expect("Create failed");
    file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    file.write_all(
        format!("{:^19} {:^0} {:^19} {:^10} {:^19} {:0} {:20}\n", "S/N", "|", "NAME OF COMMISSIONER", "|", "MINISTRY", "|", "GEOPOLITICAL ZONE")
            .as_bytes(),
    )
    .expect("Write failed");
    file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");


    for i in 0..name.len() {
        let first_item = &name[i];
        let second_item = &ministry[i];
        let third_item = &geopolitical_area[i];

        file.write_all(format!("{:^20}",i+1).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",first_item.trim()).as_bytes()).expect("Write failed");
        file.write_all(format!("{:^10}","|").as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",second_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",third_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    }
}