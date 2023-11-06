fn main() {
    let miles1:f32 = 80.0;
    let miles2:f32 = 120.0;

    //conversion
    let kilometer1:f32 = miles1 * 1.609;
    println!("80 miles to kilometer is {}",kilometer1);
    let kilometer2:f32 = miles2 * 1.609;
    println!("120 miles to kilometer is {}",kilometer2);

    //speed calculation
    let mut speed:f32;

    speed = kilometer1/2.0;
    println!("A car travels {} kilometers/hour if it goes 80 miles in 2 hours", speed);

    speed = kilometer2/2.0;
    println!("A car travels {} kilometers/hour if it goes 120 miles in 4 hours", speed);

}