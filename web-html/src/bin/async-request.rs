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

#[tokio::main]
async fn main() -> Result<()> {
    request().await?;
    Ok(())
}

async fn request() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!(
        "[async]\nstatus:{}\nheaders:\n{:#?}",
        res.status(),
        res.headers()
    );
    let body = res.text().await?;

    println!("Body:\n{}", body);
    Ok(())
}
