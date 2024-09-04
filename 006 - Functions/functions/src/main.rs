fn main() {
    let value = plus_one(3); // statement, it is an action and not return anything
    println!("{value}")
}

fn plus_one(value: i32) -> i32 {
    value + 1 // expression, because return a value
}
