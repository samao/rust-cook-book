use std::{error::Error, str::Utf8Error};

use base64::{engine::general_purpose::STANDARD, Engine};
use data_encoding::{DecodeError, HEXUPPER};
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use url::form_urlencoded::{byte_serialize, parse};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-05 15:03:09
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-05 15:05:47
 */
fn main() -> Result<(), Box<dyn Error>> {
    percent_encoding()?;
    url_encoding();
    data_encoding()?;
    base64_encoding()?;

    Ok(())
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn percent_encoding() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";
    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoding = iter.collect::<String>();
    assert_eq!(
        dbg!(&encoding),
        "confident,%20productive%20systems%20programming"
    );

    let iter = percent_decode(encoding.as_bytes());
    let decoding = dbg!(iter.decode_utf8()?);
    assert_eq!(dbg!(decoding), input);
    Ok(())
}

fn url_encoding() {
    let urlencoded = byte_serialize("What is ❤?".as_bytes()).collect::<String>();
    assert_eq!(dbg!(&urlencoded), "What+is+%E2%9D%A4%3F");
    println!("urlencoded: '{}'", urlencoded);

    let decoded = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect::<String>();
    assert_eq!(dbg!(&decoded), "What is ❤?");
    println!("decoded: '{}'", decoded);

    println!(
        "group: {:?} = {:?}",
        ["AD", "CD", "EA"].concat(),
        [[1, 2, 3], [4, 6, 3]].concat()
    );
}

fn data_encoding() -> Result<(), DecodeError> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected =
        "54686520717569636B2062726F776E20666F78206A756D7073206F76657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}

fn base64_encoding() -> Result<(), Box<dyn Error>> {
    let hello = b"hello rustaceans";
    let encoded = STANDARD.encode(hello);
    let decoded = STANDARD.decode(&encoded)?;

    println!("origin: {}", String::from_utf8(hello.to_vec())?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", String::from_utf8(decoded)?);
    Ok(())
}
