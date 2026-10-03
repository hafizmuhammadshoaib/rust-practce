mod calculator;
mod error_handling;
mod hashmap;
mod oop;
mod threads;
mod todo;
mod tokio_practice;

struct User {
    name: String,
    email: String,
    age: i32,
    status: Status,
}

#[derive(Debug)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum Status {
    Active(bool),
}

impl User {
    fn format(&self) -> String {
        let status_str = match &self.status {
            Status::Active(true) => "Active",
            Status::Active(false) => "Not Active",
        };
        format!("{} ({}) {} {}", self.name, self.age, self.email, status_str)
    }
}
fn main() {
    tokio_practice::main();
}


fn pattern_matching(number: i32) -> Weekday {
    let day = match number {
        1 => Weekday::Monday,
        2 => Weekday::Tuesday,
        3 => Weekday::Wednesday,
        4 => Weekday::Thursday,
        5 => Weekday::Friday,
        6 => Weekday::Saturday,
        7 => Weekday::Sunday,
        _ => Weekday::Sunday,
    };
    day
}

fn print_greeting_message(greeting: &mut String, person_name: &str) {
    greeting.push_str(" ");
    greeting.push_str(person_name);
}

fn data_types_as_param(a: i32, b: f64, c: bool) {
    println!("{} is i32 {} is f64 {} is bool", a, b, c);
}

fn get_fizz_buzz_or_fizzbuzz(a: i64) {
    if a % 3 == 0 && a % 5 == 0 {
        println!("FizzBuzz");
    } else if a % 3 == 0 {
        println!("Fizz");
    } else if a % 5 == 0 {
        println!("Buzz");
    }
}

fn sum_till_100_using_loop() -> i32 {
    let mut count = 1;
    let mut sum = 0;
    loop {
        println!("loop in i {}", count);
        sum += count;
        if count == 100 {
            println!(" **** breaking count {} ****", count);
            break;
        }
        count = count + 1;
    }
    return sum;
}

fn sum_till_100_using_for_loop() -> i32 {
    let mut sum = 0;
    for i in 1..101 {
        println!("for loop in i {}", i);
        sum += i
    }
    return sum;
}

fn takes_tuple_as_input_and_print(tup: &(i32, bool, &str)) {
    println!("{} {} {}", tup.0, tup.1, tup.2);
}
