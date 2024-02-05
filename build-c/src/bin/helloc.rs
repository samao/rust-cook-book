/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-05 10:35:43
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-05 10:46:14
 */

use std::{
    error::Error,
    ffi::{c_char, CString},
};

extern "C" {
    fn hello();
    fn greet(name: *const c_char);
}

fn prompt(s: &str) -> Result<String, Box<dyn Error>> {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
