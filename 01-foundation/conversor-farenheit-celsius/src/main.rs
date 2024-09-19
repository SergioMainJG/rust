use std::io;
use std::process;

fn main() {
    // let mut value_to_convert = String::new();
    loop {
        println!("Welcome!");
        println!("Please, enter the an option.");
        println!("If you want exit, please type 0 or exit");
        let mut option: String  = String::new();
        println!("1. Farenheit to Celsius");
        println!("2. Celsius to Farenheit");
        
        io::stdin()
            .read_line(&mut option )
            .expect("Please, enter a valid option");

        let option = option.trim();
        
        if option == "0" || option == "exit" {
            println!("Thanks for your visit, goodbye!");
            process::exit(0);
        }
        if option == "1" {
            println!("Please, enter the Farenheit's value to convert to Celsius");

            let mut value = String::new();
            io::stdin()
                .read_line(&mut value )
                .expect("Please, enter a valid option");
            let value: f32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("The convertions of {value} degress celsius are: {}", farenheit_to_celsius(value));
        }
        if option == "2" {
            println!("Please, enter the Celsiu's value to convert to Farenheit");

            let mut value = String::new();
            io::stdin()
                .read_line(&mut value )
                .expect("Please, enter a valid option");
            let value: f32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("The convertions of {value} degress farenheit are: {}", celsius_to_farenheit(value));
        }
    }
}

// fn clear_console(){
//         process::Command::new("clear").status().unwrap();
// }


fn farenheit_to_celsius( value: f32 ) -> f32 {
    (( value - 32.0) * 5.0 ) / 9.0 
}

fn celsius_to_farenheit( value: f32 ) -> f32{
    (( value  * 9.0 ) / 5.0 ) + 32.0 
}