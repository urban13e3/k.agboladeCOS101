// declare a structure 
struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    // intialize a structure
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };

    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };

    // pas emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

// fetch values of specific structure fields using the
// operator and print it to the console

fn display(emp:Employee) {
    println!("Name is: {}. Company is {}. Age is {}", emp.ceo, emp.company, emp.age);
}