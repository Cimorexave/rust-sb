mod todo {

    pub struct Todo {
        title: String,
        description: String,
        completed: bool,
    }
    impl Todo {
        pub fn new(title: String, description: String, completed: bool) -> Self {
            Todo { title, description, completed }
        }
    }
    pub fn run() {
        let mut choice: String = String::new();
        println!("Choose an option:");  
        println!("1. View TODO list");
        println!("2. Add a new TODO item");
        println!("3. Delete a TODO item");
        println!("4. Mark a TODO item as completed");
        
        choice = loop {
            println!("Enter your choice (1, 2, 3, or 4):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim() {
                "1" | "2" | "3" | "4" => break input,
                _ => println!("Invalid choice! Please enter 1, 2, 3, or 4."),
            }
        };

        match choice.trim() {
            "1" => {
                println!("\nTODO List:");
                for (i, todo) in todos.iter().enumerate() {
                    println!("{}. {}", i + 1, todo);
                }
            },
            "2" => {},
            "3" => {},
            _ => {},
        }
        
    }
}