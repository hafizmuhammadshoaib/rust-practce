use std::{alloc::System, io};
pub fn main(mut todos: Vec<String>) {
    show_options();

    let option = read_option_number();

    match option {
        1 => input_task(todos),
        2 => show_tasks(todos),
        3 => remove_task(todos),
        4 => std::process::exit(0),
        _ => panic!("invalid selection"),
    }
}

fn read_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    input.trim().to_string()
}

fn read_option_number() -> i32 {
    loop {
        let mut input_num = String::new();
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to readline");

        match input_num.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn show_options() {
    println!(
        "Choose an action:
1. Add Task
2. View Tasks
3. Remove Task
4. Exit"
    );

    println!("");

    println!("Your choice:");
}

fn rerun_main(mut todos: Vec<String>) {
    println!("");
    println!("");
    println!("-------------------------");
    println!("");
    main(todos);
}

fn input_task(mut todos: Vec<String>) {
    let input = read_string("Enter new task:");
    todos.push(input);
    println!("Task added ✅");
    rerun_main(todos);
}

fn show_tasks(mut todos: Vec<String>) {
    println!("📝 Your Tasks:");
    for (i, element) in todos.iter().enumerate() {
        let count = i + 1;
        println!("{} {}", count, element)
    }

    rerun_main(todos);
}

fn remove_task(mut todos: Vec<String>) {
    println!("Enter task number to remove:");
    let position = read_option_number();
    let index = (position - 1).try_into().unwrap();
    let element = todos.remove(index);

    println!("Removed {}  ❌", element);

    rerun_main(todos);
}
