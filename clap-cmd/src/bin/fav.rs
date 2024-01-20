/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-18 12:11:56
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-18 13:16:41
 */
fn main() {
    use ansi_term::Colour::*;
    let cmd = clap::Command::new("cargo")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(
            clap::command!("file").args(&[
                clap::arg!(--"manifest-path" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf)),
                clap::arg!(-n --number <LINE>),
            ]),
        );

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("file", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };

    let Some(manifest_path) = matches.get_one::<std::path::PathBuf>("manifest-path") else {
        panic!("error")
    };
    let Some(line) = matches.get_one::<String>("number") else {
        panic!("error: 1")
    };
    // println!("{manifest_path:?} =>line>: {line:?}");
    let Ok(line) = line.parse::<u64>() else {
        panic!("not a number like")
    };
    let manifest_path = manifest_path.to_str().unwrap();
    println!(
        "Your favorite number in file {} muse be {} column.",
        Red.bold().paint(manifest_path),
        Blue.paint(line.to_string())
    );
}
