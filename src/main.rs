use std::collections::VecDeque;
use std::io;

mod tasks;
use tasks::task::Task;

fn main() {
    let mut tasks = VecDeque::new();

    loop {
        println!("\nTaskMaster - Task Management System");
        println!("-----------------------------------");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Complete task");
        println!("4. Remove task");
        println!("5. Edit task");
        println!("6. Exit");
        println!("-----------------------------------");
        print!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let task = Task::new(description.trim().to_string());
                tasks.push_back(task);
            }
            "2" => {
                println!("**********************");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. [{}] {}", i + 1, if task.completed { "x" } else { " " }, task.description);
                }
                println!("**********************");
            }
            "3" => {
                println!("Enter the task number to complete:");
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).expect("Failed to read line");
                if let Ok(num) = task_number.trim().parse::<usize>() {
                    if num > 0 && num <= tasks.len() {
                        if let Some(tasks) = tasks.get_mut(num - 1) {
                            tasks.completed = true
                        }
                    } else {
                        println!("Invalid task number")
                    }
                } else {
                    println!("Invalid input")
                }
            }
            "4" => {
                println!("Enter the task number to remove:");
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).expect("Failed to read line");
                if let Ok(num) = task_number.trim().parse::<usize>() {
                    if num > 0 && num <= tasks.len() {
                        tasks.remove(num - 1);
                    } else {
                        println!("Invalid task number")
                    }
                } else {
                    println!("Invalid input")
                }
            }
            "5" => {
                println!("Enter the task number to edit:");
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).expect("Failed to read line");
                if let Ok(num) = task_number.trim().parse::<usize>() {
                    if num > 0 && num <= tasks.len() {
                        println!("Enter the new description:");
                        let mut description = String::new();
                        io::stdin().read_line(&mut description).expect("Failed to read line");
                        if let Some(task) = tasks.get_mut(num - 1) {
                            task.description = description;
                        }
                    } else {
                        println!("Invalid task number");
                    }
                } else {
                    println!("Invalid input");
                }
            }
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => println!("invalid choice, please try again.")
        }
    }
}
