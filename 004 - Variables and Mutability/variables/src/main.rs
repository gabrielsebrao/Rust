const NAME: &str = "Gabriel";

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("The value of NAME is: {NAME}");

    // shadowing
    let x = NAME;
    println!("The value of x is: {x}");
}
