use std::fmt::{Debug, Display};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-23 14:53:21
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-31 14:41:45
 */
#[derive(Debug)]
pub struct MYError(String);

impl std::error::Error for MYError {}

impl Display for MYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MYERROR: {}", self.0)
    }
}

impl From<&str> for MYError {
    fn from(value: &str) -> Self {
        MYError(value.to_owned())
    }
}

impl From<std::io::Error> for MYError {
    fn from(value: std::io::Error) -> Self {
        MYError(value.to_string())
    }
}

impl From<ring::error::Unspecified> for MYError {
    fn from(value: ring::error::Unspecified) -> Self {
        MYError(value.to_string())
    }
}

impl From<rusqlite::Error> for MYError {
    fn from(value: rusqlite::Error) -> Self {
        MYError(value.to_string())
    }
}
impl From<chrono::ParseError> for MYError {
    fn from(value: chrono::ParseError) -> Self {
        MYError(value.to_string())
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
