use std::io::{self, Write};
use std::fs::File;

fn main() {
    let x = vec!["Student Name", "Matric Number", "Department", "Level"];
    let s = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let m = vec!["ACC10211111", "ECO101110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let d = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let l = vec!["300", "100", "200", "200", "100"];

    let mut file = File::create("pau_smis.txt").expect("Failed to create file");

    write_row(&mut file, &x).expect("Write failed");

    for i in 0..s.len() {
        let row_data = vec![s[i], m[i], d[i], l[i]];
        write_row(&mut file, &row_data).expect("Write failed");
    }

    println!("\nPAU SMIS data has been saved in the 'pau_smis.txt' file");
}

fn write_row<W: Write>(file: &mut W, row: &[&str]) -> io::Result<()> {
    for (i, cell) in row.iter().enumerate() {
        write!(file, "{:<20}", cell)?; // Adjust the width as needed
        if i < row.len() - 1 {
            write!(file, "|")?;
        }
    }
    writeln!(file)?;

    Ok(())
}