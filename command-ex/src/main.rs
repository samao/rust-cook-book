use std::{error::Error, process::Command};

use regex::Regex;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-21 11:42:02
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-21 11:42:31
 */
fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;
    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(?x)([0-9a-fA-F]+) (.*)")?;
    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(10)
        .for_each(|x| println!("{:?}", x));
    Ok(())
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Commit {
    hash: String,
    message: String,
}
