/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-18 15:20:48
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-18 15:48:12
 */
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "imgs/";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| match e {
            Ok(o) => {
                // println!("start => {:?}", o.path());
                if o.path().unwrap().starts_with("imgs/rust") {
                    Some(o)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .map(|mut entry| -> std::io::Result<PathBuf> {
            match entry.path() {
                Ok(a) => {
                    let path = a
                        .strip_prefix(format!("{}/rust", prefix))
                        .unwrap()
                        .to_owned();
                    entry.unpack(&path)?;
                    Ok(path)
                }
                Err(_) => unreachable!("oops!"),
            }
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}
