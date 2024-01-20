use std::fs::File;

use flate2::{write::GzEncoder, Compression};

fn main() -> Result<(), std::io::Error> {
    println!("zip app");
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("imgs", "./0909")?;
    Ok(())
}
