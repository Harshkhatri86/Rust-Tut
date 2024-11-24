use std::io::{self, Write};

struct TodoItem {
    id: u32,
    task: String,
    completed: bool,
}

impl TodoItem {
    fn new(id: u32, task: String) -> Self {
        Self {
            id,
            task,
            completed: false,
        }
    }
}

fn add_task(tasks: &mut Vec<TodoItem>, id: u32, task: String) {
    let new_task = TodoItem::new(id, task);
    tasks.push(new_task);
}

fn list_task(tasks: &Vec<TodoItem>) {
    for task in tasks {
        println!(
            "ID : {} , Task : {} , Completed : {}",
            task.id, task.task, task.completed
        );
    }
}

fn mark_completed(task: &mut Vec<TodoItem>, id: u32) {
    if let Some(task) = task.iter_mut().find(|t| t.id == id) {
        task.completed = true;
    }
}

fn main() {
    let mut tasks: Vec<TodoItem> = Vec::new();
    loop {
        println!("Choose an option : add, list , completed , exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "add" => {
                println!("Enter Task : ");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                let id = tasks.len() as u32 + 1;
                add_task(&mut tasks, id, task.trim().to_string());
            }
            "list" => {
                list_task(&tasks);
            }
            "complete" => {
                println!("Enter the task Id to complete");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    mark_completed(&mut tasks, id);
                } else {
                    println!("invalid Id ");
                }
            }
            "exit" => break,
            _ => println!("Invalid option"),
        }
    }
}
