use std::io;
pub fn main() {
    let num1 = read_number("Enter first number: ");
    let num2 = read_number("Enter 2nd number: ");
    println!(
        "Choose an operation:
+  Addition
-  Subtraction
*  Multiplication
/  Division"
    );
    let operator: String = read_string();

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "/" => num1 / num2,
        "*" => num1 * num2,
        _ => panic!("Invalid operator {}", operator),
    };
    println!(
        "You entered: {} and {} and result is {}",
        num1, num2, result
    );

    println!("Do you want to perform another calculation? (y/n):");
    let yes_or_no = read_string();
    if yes_or_no == "y" {
        main()
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn read_string() -> String {
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    operator.trim().to_string()
}
