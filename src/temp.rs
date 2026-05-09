pub(super) mod temp_convertor {
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    // main function for terminal application
    pub fn run() -> bool {

        use std::io;
        let mut choice = String::new();
            
            println!("Temperature Converter");
            println!("1. Celsius to Fahrenheit");
            println!("2. Fahrenheit to Celsius");
            println!("3. Exit");
            println!("Enter your choice (1 or 2 or 3):");

            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: u32 = choice.trim().parse().expect("Please enter a number");
            
                match choice {
                    1 => {
                        println!("Enter temperature in Celsius:");
                        let mut celsius = String::new();
                        io::stdin().read_line(&mut celsius).expect("Failed to read line");
                        let celsius: f64 = celsius.trim().parse().expect("Please enter a number");
                        let fahrenheit = celsius_to_fahrenheit(celsius);
                        println!("{celsius}°C is {fahrenheit}°F");
                    },
                    2 => {
                        println!("Enter temperature in Fahrenheit:");
                        let mut fahrenheit = String::new();
                        io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
                        let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please enter a number");
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