fn main() {
    println!("Hello, World");
    let mut greeting = "test";
    greeting = "hello";
    // panic!("end {}", greeting);
    println!("{}", greeting);
    mainn();
}

fn mainn() {
    let x = 1.1;
    let mut y = 2.2;
    // y = "two point two";  // will have error, because can not change type 
    println!("{}", x * y);
}

// Integres
static nighty: i32 = 90;
static negative_five: i32 = -5;
static one_thousand: i32 = 1_1000;


