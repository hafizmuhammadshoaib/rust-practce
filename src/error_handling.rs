use core::fmt;
use std::{fmt::Debug, fs::File, io::Read, num::ParseIntError, ptr::null};
use thiserror::Error;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    EmptyInput,
}


#[derive(Error, Debug)]
enum LoginError {
    #[error("User Not found")]
    UserNotFound,
    #[error("Wrong Password provided")]
    WrongPassword,
     #[error("Empty input fields")]
    EmptyFields,
}

// impl fmt::Display for LoginError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             LoginError::UserNotFound => write!(f, "User Not found"),
//             LoginError::WrongPassword => write!(f, "Wrong Password provided"),
//             LoginError::EmptyFields => write!(f, "Empty input fields"),
//         }
//     }
// }

// impl std::error::Error for LoginError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             LoginError::EmptyFields => None,
//             LoginError::UserNotFound => None,
//             LoginError::WrongPassword => None,
//         }
//     }
// }

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::EmptyInput => write!(f, "Empty input error"),
        }
    }
}

// Implement std::error::Error to make it a proper error type
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
            AppError::EmptyInput => None,
        }
    }
}

// Implement From traits for automatic conversion
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> AppError {
        AppError::Parse(err)
    }
}

pub fn main() {
    

    match login("","") {
        Ok(())=> println!("login successful!"),
        Err(e) => println!("{}",e)
    }

}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;
    } else {
        return Some(a / b);
    }
}

fn find_word(words: &Vec<String>, target: &str) -> Option<usize> {
    for (index, word) in words.iter().enumerate() {
        if String::from(target).eq(word) {
            return Some(index);
        } else {
            return None;
        }
    }
    return None;
}
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut str = String::new();
    File::open(path)?.read_to_string(&mut str)?;
    Ok(str)
}

fn read_and_parse() -> Result<i32, AppError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?; // Automatically converted to AppError::Io

    if input.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }

    let num: i32 = input.trim().parse()?; // Automatically converted to AppError::Parse
    Ok(num)
}

fn login(username: &str, password: &str) -> Result<(), LoginError> {
    if username.eq("") && password.eq(""){
        return Err(LoginError::EmptyFields);
    }
    if !username.eq("shoaib") {
        return Err(LoginError::UserNotFound);
    } if !password.eq("12345678"){
        return Err(LoginError::WrongPassword)
    }
    return Ok(());
}
