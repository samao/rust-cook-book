use std::io::Read;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-23 10:27:37
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-23 10:29:44
 */
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// #[tokio::main]
fn main() -> Result<()> {
    sync_request()?;
    Ok(())
}

fn sync_request() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();

    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: \n{}", body);
    Ok(())
}
