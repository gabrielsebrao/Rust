fn main() {

    let mut first_temperature: String = String::new();
    let mut final_temperature: String = String::new();
    let mut first_temperature_value: String = String::new();
    let final_temperature_value: f64;

    println!("Conversor de Temperatura");

    println!("Escolha a primeira temperatura: 
[1] - Celsius
[2] - Fahrenheit
[3] - Kelvin");

    std::io::stdin().read_line(&mut first_temperature).expect("Não pode ser lido!");
    let first_temperature: char = first_temperature.trim().parse().expect("Não é um dos digitos permitidos!");

    if !("123".contains(first_temperature)) {
        println!("Por favor, insira uma das opções válidas!");
        return;
    }

    println!("Insira o valor da temperatura:");
    std::io::stdin().read_line(&mut first_temperature_value).expect("Não pode ser lido!");
    let first_temperature_value: f64 = first_temperature_value.trim().parse().expect("Não é um número!");

    println!("Escolha a segunda temperatura: 
[1] - Celsius
[2] - Fahrenheit
[3] - Kelvin");

    std::io::stdin().read_line(&mut final_temperature).expect("Não pode ser lido!");
    let final_temperature: char = final_temperature.trim().parse().expect("Não é um dos digitos permitidos!");

    if !("123".contains(final_temperature)) {
        println!("Por favor, insira uma das opções válidas!");
        return;
    }

    if first_temperature == final_temperature {
        println!("Por favor, esolha temperaturas diferentes!");
        return;
    }

    final_temperature_value = match(first_temperature, final_temperature) {
        ('1', '2') => { first_temperature_value * 1.8 + 32.0 }
        ('2', '1') => { (first_temperature_value - 32.0) * 5.0/9.0 }
        ('1', '3') => { first_temperature_value + 273.15 }
        ('3', '1') => { first_temperature_value - 273.15 }
        ('2', '3') => { (first_temperature_value - 32.0) * 5.0/9.0 + 273.15 }
        ('3', '2') => { (first_temperature_value - 273.15) * 9.0/5.0 + 32.0 }
        (_null, _other_null) => { 0.0 }
    };
        
    println!("Final Temperature: {final_temperature_value}");
}
