use std::{fs::File, io::Write};

use bytes::Buf;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = tempdir::TempDir::new("rust.download")?;
    let target = "https://upstatic.qiecdn.com/upload/712fd751e1391799beef9189db55141d.jpg?op=imageView2&mode=2&width=1440";
    let response = reqwest::get(target).await?;

    let mut dest: File = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|sqgments| sqgments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };

    println!("下载完成: {}", response.status());
    let content = response.bytes().await?;
    dest.write_all(content.chunk())?;

    // 以下代码有误，text默认转utf-8会丢失数据
    // let content = response.text().await?;
    // copy(&mut content.as_bytes(), &mut dest)?
    // use std::time::Duration;
    // use std::thread::sleep;
    // sleep(Duration::from_secs(120));
    Ok(())
}
