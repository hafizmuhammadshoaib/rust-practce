// let post_summary = Post {};
// let news_article_summary = NewsArticle {};
// let mut summaries: Vec<&dyn Summary> = vec![&post_summary, &news_article_summary];

use std::f64::consts::PI;

// for (_, element) in summaries.iter().enumerate() {
//     println!("{}", element.summarize());
// }
struct User {
    username: String,
    email: String,
    age: i32,
}

impl User {
    fn greet(&self) {
        println!("Hello, I'm [{}], [{}] years old!", self.username, self.age);
    }
}

struct UserBuilder {
    username: String,
    email: String,
    age: i32,
}

impl UserBuilder {
    fn new() -> Self {
        UserBuilder {
            username: String::new(),
            email: String::new(),
            age: 0,
        }
    }

    fn username(mut self, username: String) -> Self {
        self.username = username;
        return self;
    }

    fn email(mut self, email: String) -> Self {
        self.email = email;
        return self;
    }

    fn age(mut self, age: i32) -> Self {
        self.age = age;
        return self;
    }

    fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
        }
    }
}
trait Summary {
    fn summarize(&self) -> String;
}

enum States {
    Draft,
    Pending,
    Published,
}

struct Post {
    state: States,
}

impl Post {
    fn update_state(&mut self, state: States) -> () {
        self.state = state
    }

    fn print(&self) -> String {
        match self.state {
            States::Draft => String::from("Draft"),
            States::Pending => String::from("Pending"),
            States::Published => String::from("Published"),
        }
    }
}

struct NewsArticle {}

impl Summary for Post {
    fn summarize(&self) -> String {
        String::from("dasf")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        String::from("dsafesf")
    }
}

trait Animal {
    fn speak(&self) -> String;
}

struct Dog {}

struct Cat {}

impl Animal for Dog {
    fn speak(&self) -> String {
        String::from("Boh Boh!")
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        String::from("Meow Meow")
    }
}

trait Shape {
    fn area(&self) -> f64;

    fn describe(&self) -> &'static str {
        return "this is shape";
    }
}
struct Circle {
    x: i32,
    y: i32,
    radius: f64,
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius.powi(2))
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

pub fn main() {
    let user = UserBuilder::new()
        .username(String::from("Shoaib"))
        .email(String::from("shoaibsilat9@gmail.com"))
        .age(32)
        .build();

    user.greet();
}
