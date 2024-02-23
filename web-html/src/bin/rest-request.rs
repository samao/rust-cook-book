/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-23 11:02:06
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-23 11:04:50
 */

use std::{collections::HashMap, error::Error, time::Duration};

use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Client, ClientBuilder,
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
struct HeadersEcho {
    headers: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;
    let response = Client::new()
        .get(url)
        .header(USER_AGENT, "Rust-test")
        .header("fe", "qie".to_string())
        .header(AUTHORIZATION, format!("Bearer {}", "this is a token str"))
        .send()
        .await?;

    println!("URL: {}", response.url().as_str());
    let out: HeadersEcho = response.json().await?;
    println!("Headers: {:?}", out.headers);
    println!("{:?}", out);

    git_api().await?;

    check_url().await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct User {
    login: String,
    id: u32,
    r#type: String,
}

//https://api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers
async fn git_api() -> Result<(), Box<dyn Error>> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("GITHUB: {}", request_url);

    let response = reqwest::Client::new()
        .get(&request_url)
        .header(USER_AGENT, "rust-client") // github 要求有ua
        .send()
        .await?;
    // println!("{:?}", response.text().await?);
    if response.status().is_success() {
        let users: Vec<User> = response.json().await?;
        println!("{:#?}", users);
    } else {
        println!("请求失败: {:?}", response.status());
    }
    Ok(())
}

async fn check_url() -> Result<(), Box<dyn Error>> {
    let user = "samao";

    let request_url = format!("https://api.github.com/users/{}", user);
    println!("请求： {}", request_url);
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client
        .head(&request_url)
        .header(USER_AGENT, "rust-client")
        .send()
        .await?;
    if response.status().is_success() {
        println!("{} is a user", user);

        if let Some(size) = response.headers().get("content-length") {
            println!("document size: {}", size.to_str()?.parse::<u64>()?);
        }
    } else {
        println!("{} is not a user #{}", user, response.status());
    }

    Ok(())
}
