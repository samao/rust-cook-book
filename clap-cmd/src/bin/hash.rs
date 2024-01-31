/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-23 15:21:48
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-23 15:58:14
 */
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    num::NonZeroU32,
};

use data_encoding::HEXUPPER;
use ring::{
    digest::{self, Context, Digest, SHA256},
    hmac, pbkdf2,
    rand::{self, SecureRandom},
};

fn main() -> clap_cmd::Result<()> {
    let path = "file.txt";
    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);
    let digest = sha256_digest(&mut reader)?;
    let digest_full = sha256_digest_full(&mut reader)?;

    println!(
        "SHA-256 digest 1024 is {}",
        HEXUPPER.encode(digest.as_ref())
    );
    println!(
        "SHA-256 digest full is {}",
        HEXUPPER.encode(digest_full.as_ref())
    );

    // hmac 签名验证
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    let message = "Legitimate and important message.";
    // 签名
    let signature = hmac::sign(&key, message.as_bytes());
    println!("签名<{:?}> = {:?}", message, signature);
    // 验证
    if let Err(msg) = hmac::verify(&key, message.as_bytes(), signature.as_ref()) {
        println!("验证失败: {}", msg);
    } else {
        println!("验证通过");
    }

    encrypt()?;

    if let Ok(err) = read_count("count.txt") {
        println!("读取计数={:#010X}", err);

        let a = 0xF34123;
        let b = format!("{:#010X}", a);
        println!("CURRENT: {}", b);
    } else {
        println!("读取技术失败");
    }

    Ok(())
}

fn read_count(path: &str) -> Result<i64, Box<dyn Error>> {
    let mut file = File::create(path)?;
    write!(file, "998736")?;
    let mut count = String::new();
    File::open(path)?.read_to_string(&mut count)?;
    let count = count.parse::<i64>()?;
    Ok(count)
}

// 1024 bytes hash
fn sha256_digest<R: Read>(reader: &mut R) -> clap_cmd::Result<Digest> {
    let mut context = Context::new(&SHA256);

    let mut buf = [0; 1024];
    // println!("before: {:?}", buf);
    loop {
        let count = reader.read(&mut buf)?;
        if count == 0 {
            break;
        }
        context.update(&buf[..count]);
    }
    // println!("after: {:?}", buf);
    Ok(context.finish())
}

// full bytes hash
fn sha256_digest_full<R: Read>(reader: &mut R) -> clap_cmd::Result<Digest> {
    let mut context = Context::new(&SHA256);

    let mut data = vec![];
    let _ = reader.read_to_end(&mut data)?;
    context.update(&data[..]);
    Ok(context.finish())
}

fn encrypt() -> clap_cmd::Result<()> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password = "Guess My If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );

    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());
    Ok(())
}
