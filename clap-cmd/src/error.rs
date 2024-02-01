use std::fmt::{Debug, Display};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-23 14:53:21
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-31 14:41:45
 */
#[derive(Debug)]
pub struct MyError(String);

impl std::error::Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MYERROR: {}", self.0)
    }
}

impl From<&str> for MyError {
    fn from(value: &str) -> Self {
        MyError(value.to_owned())
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        MyError(value.to_string())
    }
}

impl From<ring::error::Unspecified> for MyError {
    fn from(value: ring::error::Unspecified) -> Self {
        MyError(value.to_string())
    }
}

impl From<rusqlite::Error> for MyError {
    fn from(value: rusqlite::Error) -> Self {
        MyError(value.to_string())
    }
}
impl From<chrono::ParseError> for MyError {
    fn from(value: chrono::ParseError) -> Self {
        MyError(value.to_string())
    }
}

impl From<log::SetLoggerError> for MyError {
    fn from(value: log::SetLoggerError) -> Self {
        MyError(value.to_string())
    }
}

impl From<Box<String>> for MyError {
    fn from(value: Box<String>) -> Self {
        MyError(*value)
    }
}

trait Walk {
    /// GUUUO
    fn walk(&self) {
        println!("I can Walk");
    }
}
impl<T: Debug> Walk for T {}

#[derive(Debug)]
struct Good;

pub fn say() {
    let a = Good;
    a.walk();

    let a = "Manny";
    a.walk();

    let a = 1000_u16;
    a.walk();

    let a = Box::new("So so");
    a.walk();

    bye(Goods("i have something new"));
}

#[derive(Debug)]
struct Goods<'a>(&'a str);

fn bye(Goods(value): Goods) {
    println!("fn match = {}", value);
}
