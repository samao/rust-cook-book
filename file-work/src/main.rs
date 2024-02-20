/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-20 15:13:15
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-20 16:57:46
 */
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use glob::{glob, glob_with, MatchOptions};
use same_file::is_same_file;
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Box<dyn Error>> {
    println!("*** contains_loop ***");
    assert_eq!(
        contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
        Some((
            PathBuf::from("/tmp/foo"),
            PathBuf::from("/tmp/foo/bar/baz/qux")
        ))
    );
    find_24_modified()?;
    find_repeat_file();
    skip_hidden_or_release();
    calc_dir_size();
    find_speci_file()?;
    config_match_file()?;
    println!(
        "Number of logical cores is {}, physical: {}",
        num_cpus::get(),
        num_cpus::get_physical()
    );
    // let total = [1, 2, 3, 4, 5].iter().fold(20, |cur, item| cur + item);
    // println!("fold is {}", total);
    Ok(())
}

fn config_match_file() -> Result<(), Box<dyn Error>> {
    println!("*** config_match_file ***");
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with("../**/cargo.toml", options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}

fn find_speci_file() -> Result<(), Box<dyn Error>> {
    println!("*** find_speci_file ***");
    for entry in glob("../**/src/*.rs")? {
        println!("rust source code: {}", entry?.display());
    }
    Ok(())
}

fn calc_dir_size() {
    println!("*** calc_dir_size ***");
    let total_size = WalkDir::new("..")
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|e| e.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes", total_size);
}

fn skip_hidden_or_release() {
    println!("*** skip hidden or release file ***");
    WalkDir::new("..")
        .into_iter()
        .filter_entry(|e| is_not_hidden_or_release(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
}

fn is_not_hidden_or_release(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| (entry.depth() == 0 || !s.starts_with('.')) && !s.contains("target"))
        .unwrap_or(false)
}

fn find_repeat_file() {
    println!("*** find repeat file ***");
    let mut filenames = HashMap::new();

    for entry in WalkDir::new("..")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && !e.path().to_string_lossy().contains("target"))
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}

fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();

    let mut path_buf = path.to_path_buf();

    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = contains_loop(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    Ok(None)
}

fn find_24_modified() -> Result<(), Box<dyn Error>> {
    println!("*** find_24_modified ***");
    let current_dir = env::current_dir()?;
    println!("Entries modified in the last 24 hours in {:?}", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last Modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }
    Ok(())
}
