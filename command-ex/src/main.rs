use std::{
    collections::HashSet,
    env,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Error as IOError, ErrorKind, Write},
    process::{Command, Stdio},
};

use regex::Regex;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-21 11:42:02
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-21 11:42:31
 */
fn main() -> Result<(), Box<dyn Error>> {
    git_commit()?;
    python()?;
    find_max_size_file()?;
    command_to_file()?;
    read_env_vars()?;
    persist_read()?;
    Ok(())
}

fn read_env_vars() -> Result<(), Box<dyn Error>> {
    let config_path = env::var("CONFIG").unwrap_or("config.default".to_string());

    let config = fs::read_to_string(config_path)?;

    println!("Config: {}", config);
    Ok(())
}

fn persist_read() -> Result<(), Box<dyn Error>> {
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| IOError::new(ErrorKind::Other, "Could not capture standard output."))?;
    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));
    Ok(())
}

fn command_to_file() -> Result<(), Box<dyn Error>> {
    let outputs = File::create("log.txt")?;
    let errors = outputs.try_clone()?;
    Command::new("ls")
        .args(&["-R", "-S", "-l", "../build-c/src"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;
    Ok(())
}

fn find_max_size_file() -> Result<(), Box<dyn Error>> {
    let directory = env::current_dir()?;
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()?;
        du_output_child.wait()?;

        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child = Command::new("head")
                .args(&["-n", "10"])
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()?;
            let head_stdout = head_output_child.wait_with_output()?;
            sort_output_child.wait()?;

            println!(
                "Top 10 biggest files and directories in '{}': \n{}",
                directory.display(),
                String::from_utf8(head_stdout.stdout).unwrap()
            );
        }
    }

    Ok(())
}

fn python() -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("python")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("child process stdin has not been captured!")?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        error_chain::bail!("External command failed: \n {}", err)
    }
}

fn git_commit() -> Result<(), Box<dyn Error>> {
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
        .take(3)
        .for_each(|x| println!("{:?}", x));
    Ok(())
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Commit {
    hash: String,
    message: String,
}
