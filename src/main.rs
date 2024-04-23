mod todo;
use todo::{read_todos_from_file, write_todos_to_file, Todo};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <action> [title]");
        return;
    }

    let action = &args[1];
    let title = args.get(2).cloned();

    let mut todos = read_todos_from_file();

    match action.as_str() {
        "add" => {
            if let Some(title) = title {
                let todo = Todo::new(title);
                todos.push(todo);
            } else {
                println!("Please provide a title for the todo.");
                return;
            }
        }
        "complete" => {
            if let Some(title) = title {
                if let Some(todo) = todos.iter_mut().find(|x| x.title == title) {
                    todo.complete();
                } else {
                    println!("To-do not found: {}", title);
                    return;
                }
            } else {
                println!("Please provide a title for the todo to complete.");
                return;
            }
        }
        "list" => {
            for todo in &todos {
                println!(
                    "{} - {}",
                    todo.title,
                    if todo.completed { "✔" } else { " " }
                );
            }
            return;
        }
        _ => {
            println!("Unknown action: {}", action);
            return;
        }
    }
    write_todos_to_file(&todos);
}
