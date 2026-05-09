pub(super) mod temp_convertor {
    fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    pub fn run() -> bool {
        use std::io;
        let mut choice = String::new();
            
        println!("Temperature Converter");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");
            
        let choice: u32 = loop {
            println!("Enter your choice (1 or 2 or 3):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<u32>() {
                Ok(val) if (1..=3).contains(&val) => break val,
                _ => println!("Invalid choice! Please enter 1, 2, or 3."),
            }
        };
            
        match choice {
            1 => {
                let celsius: f64 = loop {
                    println!("Enter temperature in Celsius:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    match input.trim().parse() {
                        Ok(val) => break val,
                        Err(_) => println!("Invalid number. Please enter a valid temperature."),
                    }
                };
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{celsius}°C is {fahrenheit}°F");
            },
            2 => {
                let fahrenheit: f64 = loop {
                    println!("Enter temperature in Fahrenheit:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    match input.trim().parse() {
                        Ok(val) => break val,
                        Err(_) => println!("Invalid number. Please enter a valid temperature."),
                    }
                };
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{fahrenheit}°F is {celsius}°C");
            },
            3 => {  
                println!("Exiting...");
                return false;
            },
            _ => {
                println!("Invalid choice! Please enter 1 or 2 or 3.")
            },
        }
        return true;
    }
}
