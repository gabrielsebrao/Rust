fn main() {
    let age: i32 = 19;

    if age < 18 { // the type accept is BOOLEAN 
        println!("It is a minor.")
    } else {
        println!("It is an adult.")
    }

    let is_adult: bool = if age < 18 { false } else { true }; // can return only a type, in this case boolean
    println!("They are an adult: {is_adult}");

    let mut num: i32 = 0;
    'first: loop {
        num += 10;

        println!("{num}");

        if num >=50 {
            break 'first; // to avoid ambiguous breaks
        }
    }

    while num < 100 {
        num += 10;
        println!("{num}");
    }

    let list: [i32; 5] = [1, 2, 3, 4, 5];

    for element in list {
        println!("{element}");
    }

    for element in 0..5 {
        println!("The position {element} is {:?}", list[element])
    }
}
