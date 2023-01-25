use std::io;

fn main() {
    
    // main loop
    loop {

        let mut user_input: String = String::new();
        let mut user_temp_input: String = String::new();

        //Ask user to what he wants to convert
        println!("Type 'C' to convert Fahrenheit to Celsius or type 'F' to convert Celsius to Fahrenheit.");
        println!("If you want to exit simply type 'exit'.");
        
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the input.");
        
        //trimming the user input
        let user_input: String = user_input.trim().to_string();
        
        //Ceck if user wants to exit
        if user_input == r#"exit"# {
            println!("Bye.");
            break;
        }

        //Ask user for the temperature
        println!("Input your temperature:");
        io::stdin()
            .read_line(&mut user_temp_input)
            .expect("Failed to read the input.");
        
        //Make sure the input is a number
        let user_temp_input: f64 = match user_temp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Do the Conversions
        if user_input == r#"C"# {
            let output: f64 = f_to_c(user_temp_input);
            println!("Your temperature in celsius is: {output}");
            continue;
        } else if user_input == r#"F"# {
            let output: f64 = c_to_f(user_temp_input);
            println!("Your temperature in fahrenheit is: {output}");
            continue;
        } else {
            println!("Invalid Input");
            continue;
        }
    };
}
//This function converts fahrenheit to celsius
fn f_to_c(temp: f64) -> f64 {
    let x: f64 = temp - 32.0; 
    return x / 1.8;
}
//This function converts celsius to fahrenheit
fn c_to_f(temp: f64) -> f64 {
    return temp * 1.8 + 32.0
}