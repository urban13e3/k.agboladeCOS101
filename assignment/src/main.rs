use std::io::{self, Write};

struct Organization {
    company: String,
    date: i32,
    assets: f32,
    liabilities: f32,
}

impl Organization {
    fn percent_level(&self) -> f32 {
        ((self.assets - self.liabilities) * 100.0) / self.assets
    }
}

fn main() {
    let organizations = vec![
        Organization { company: "cadb".to_string(), date: 1965, assets: 15_000_000.0, liabilities: 5_500_000.0 },
        Organization { company: "cham".to_string(), date: 1974, assets: 25_000_000.0, liabilities: 8_000_000.0 },
        Organization { company: "dang".to_string(), date: 1970, assets: 18_000_000.0, liabilities: 10_000_000.0},
        Organization { company: "flou".to_string(), date: 1960, assets: 32_000_000.0, liabilities: 4_000_000.0},
        Organization { company: "nest".to_string(), date: 1961, assets: 8_000_000.0, liabilities: 1_500_000.0},
        Organization { company: "unil".to_string(), date: 1923, assets: 37_000_000.0, liabilities: 11_000_000.0},
        Organization { company: "hone".to_string(), date: 1906, assets: 34_000_000.0, liabilities: 9_000_000.0},
        Organization { company: "nige".to_string(), date: 1946, assets: 30_000_000.0, liabilities: 12_000_000.0}, 
        
    ];

    let mut file = std::fs::File::create("Companies_Information.txt").expect("Failed to create file");
    
    let mut high_assets_file = std::fs::File::create("High_Assets_Companies.txt").expect("Failed to create file");
    
    let mut low_liabilities_file = std::fs::File::create("Low_Liabilities_Companies.txt").expect("Failed to create file");

    let header = vec!["Company", "Leverages(%)", "Date", "Assets", "Liabilities"];
    writeln!(file, "{}\t\t{}\t\t{}\t\t{}\t\t{}", header[0], header[1], header[2], header[3], header[4])
        .expect("Failed to write to file");
    writeln!(file, "\n").expect("Failed to write to file");

    println!("Input username: ");
    let mut input_username = String::new();
    io::stdin().read_line(&mut input_username).expect("Failed to read input");
    let input_username = input_username.trim();

    println!("Input password: ");
    let mut input_password = String::new();
    io::stdin().read_line(&mut input_password).expect("Failed to read input");
    let input_password = input_password.trim();

    let mut user_authenticated = false;

    for org in &organizations {
        if input_username == &org.company[..4] && input_password.len() >= 3 && input_password.len() <= 8 {
            user_authenticated = true;

            writeln!(file, "{}\t\t{:.1}\t\t{}\t\t{}\t\t{}", 
                org.company, org.percent_level(), org.date, org.assets, org.liabilities)
                .expect("Failed to write to file");

            
            if org.assets > 20_000_000.0 {
                writeln!(high_assets_file, "\nCompany Name: {}\nCompany Assets: {}\nCompany Leverage: {:.1}%", 
                    org.company, org.assets, org.percent_level())
                    .expect("Failed to write to file");
            }

            
            if org.liabilities < 10_000_000.0 {
                let five_percent_leverage = org.percent_level() * 0.05;
                writeln!(low_liabilities_file, "\nCompany Name: {}\nCompany Liabilities: {}\n5% of Percentage Leverage: {:.1}%", 
                    org.company, org.liabilities, five_percent_leverage)
                    .expect("Failed to write to file");
            }

            break;
        }
    }



    if !user_authenticated {
        writeln!(file, "Access denied. Invalid username or password.")
            .expect("Failed to write to file");
    }
}
