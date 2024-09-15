fn main() {

    let mut actual_number: i32;
    let mut last_number: i32 = 1;
    let mut before_last_number: i32 = 1;

    println!("Those are the first 10 numbers in Fibonacci Sequence!");

    for c in 0..10 {
        if c==0 || c==1 { print!("1, "); continue }

        actual_number = last_number + before_last_number;
        print!("{actual_number}");

        before_last_number = last_number;
        last_number = actual_number;

        if c == 9 { break }

        print!(", ")
    }
}
