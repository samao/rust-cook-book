use std::{
    fs::{self, File},
    io::Read,
};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-06 13:26:57
 * @Last Modified by:   idzeir
 * @Last Modified time: 2024-02-06 13:26:57
 */
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    use std::io::Write;

    let mut file = File::create("./proc/downtime")?;
    write!(file, r#"{{"name": "{}", "age": {}}}"#, "JONE", 35)?;
    let json = fs::read_to_string("./proc/downtime")?;
    let json = json
        .split(&['{', '}', '\"', ':', ',', ' '][..])
        .filter(|x| !x.is_empty());
    println!("{:?}", json.collect::<Vec<_>>());

    let mut uptime = String::new();
    File::open("./proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split(".")
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    }
}
