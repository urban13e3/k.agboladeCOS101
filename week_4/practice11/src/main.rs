fn main(){
    let a:i32 = 2;  //Bit presentaton 10
    let b:i32 = 3;  //Bit presentaton 11

    let mut result:i32;

    result = a & b;
    println!("(a & b) => {}",result);

    result = a | b;
    println!("(a | b) => {}",result);

    result = a ^ b;
    println!("(a ^ b) => {}",result);

    result = !b;
    println!("(!b) => {}",result);

    result = a << b;
    println!("(a << b) => {}",result);

    result = a >> b;
    println!("(a >> b) => {}",result);
}
