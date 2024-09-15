fn main() {
    let name: String = String::from("Gabriel");
    print_name(&name);
    print_name2(name.clone())
}

fn print_name(name: &String) {
    println!("{name}");
}

fn print_name2(name: String) {
    println!("{name}");
}