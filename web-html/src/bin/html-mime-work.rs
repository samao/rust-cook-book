/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-22 16:35:50
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-22 16:38:59
 */
use error_chain::error_chain;
use mime::Mime;
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Header(reqwest::header::ToStrError);
        Mime(mime::FromStrError);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://live.qq.com/api/index").await?;
    let res_headers = response.headers();

    match res_headers.get(CONTENT_TYPE) {
        None => println!("The response does not contain a Content-type header."),
        Some(ct) => {
            let content_type = Mime::from_str(ct.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                (mime::APPLICATION, mime::JSON) => "a JSON",
                _ => "neither text nor image",
            };

            println!("The response contains {}", media_type);
        }
    }
    Ok(())
}
