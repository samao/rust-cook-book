/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-20 17:59:08
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-20 18:16:20
 */
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        println!("init PRIVLEGES");
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
    #[derive(Debug)]
    static ref PRIVI: HashMap<&'static str, Vec<&'static str>> =
        HashMap::from([("James", vec!["user", "admin"]), ("Jim", vec!["user"])]);
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}

fn main() {
    println!("run main");
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);

    show_access("Jim");

    println!("{:?}", PRIVI.iter());
}
