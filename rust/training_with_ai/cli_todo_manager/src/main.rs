use std::{fmt, io::stdin};

enum TaskStatus {
    Done,
    Postponed,
    Active,
}

struct Task {
    id: String,
    title: String,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: String, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: TaskStatus::Active,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.title, self.description)
    }
}

enum UserAction {
    Add(Task),
    Delete { id: String },
    EditTitle { id: String, new_title: String },
    EditDescription { id: String, new_description: String },
    EditStatus { id: String, new_status: TaskStatus },
    ListTasks,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    let mut user_action_input = String::new();

    let valid_inputs = [
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];

    loop {
        println!("What do you want to do?");
        println!("1. Add new a task");
        println!("2. Delete an existing task");
        println!("3. Edit an existing task");
        println!("4. See all tasks");

        while !valid_inputs.contains(&user_action_input) {
            user_action_input = String::new();
            stdin()
                .read_line(&mut user_action_input)
                .expect("Failed to read line");
            user_action_input.pop();
            if !valid_inputs.contains(&user_action_input) {
                println!("Enter an option between 1 and 4");
            }
        }

        match user_action_input.as_str() {
            "1" => {
                let mut title = String::new();
                let mut desc = String::new();
                println!("Enter title:");
                stdin().read_line(&mut title).expect("Failed to read");
                println!("Enter description:");
                stdin().read_line(&mut desc).expect("Failed to read");

                let next_id = tasks
                    .last()
                    .map(|t| t.id.clone())
                    .unwrap_or("1".to_string());

                let new_task = Task::new(next_id, title, desc);
                tasks.push(new_task);
                user_action_input = String::new();
            }
            "2" => println!("Deleting a task"),
            "3" => println!("Editing a task"),
            "4" => {
                tasks.iter().for_each(|t| println!("{}", t));
                user_action_input = String::new();
            }
            _ => println!("Invalid input"),
        };
    }
}
