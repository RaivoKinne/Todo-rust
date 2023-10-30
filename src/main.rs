use std::io;

struct Todo {
    title: String,
}

impl Todo {
    fn new(title: String) -> Todo {
        Todo { title }
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("Enter 1 to add a Todo, 2 to exit:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter Todo: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let todo = Todo::new(title.trim().to_string());
                todos.push(todo);
                println!("Todo added successfully.");
            }
            "2" => {
                println!("Exiting program.");
                println!("Todos:");
                for todo in todos {
                    println!("{}", todo.title)
                }
                break;
            }
            _ => {
                println!("Invalid input. Please enter 1 or 2.");
            }
        }
    }
}
