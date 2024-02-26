use std::fmt::Debug;

use bytes::Buf;
/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-26 10:24:34
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-26 11:15:38
 */
use encoding_rs::*;

fn main() {
    let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}1";
    let bytes_data = b"\x83n\x83\x8D\x81[\x81E\x83\x8F\x81[\x83\x8B\x83h1";
    let (cow, encoding_used, had_error) = SHIFT_JIS.decode(bytes_data);

    println!("1.{:?}", bytes_data);

    let mut bytes_data_u16 = [0u16; 9];

    // 偶数会舍弃末尾单元数
    let ite = bytes_data.chunks_exact(2);
    println!("奇数元素，剩余最后: {:?}", ite.remainder());
    for (i, chunk) in ite.clone().enumerate() {
        let bytes_d = [chunk[0], chunk[1]];
        bytes_data_u16[i] = u16::from_be_bytes(bytes_d);
    }
    println!("2.{:0x?}", bytes_data_u16);

    // use bytes::Buf;
    let mut reader = &bytes_data[..];
    let mut b16v = vec![];
    loop {
        if reader.remaining() == 0 {
            break;
        }
        if reader.remaining() == 1 {
            // print!("{:0x?}\n", reader.get_u8() as u16);
            b16v.push(reader.get_u8() as u16);
            break;
        }
        // print!("{:0x?}, ", reader.get_u16());
        b16v.push(reader.get_u16())
    }
    println!("3.{:0x?}", b16v);

    println!("{} == {:?}", expectation, &cow.get(..).unwrap_or(""));
    println!("{:?}, {}", encoding_used, had_error);

    // let expectation = "✈️💃🏻💐你好🔫";
    // let (bytes, _, _) = GBK.encode(expectation);
    // unsafe {
    let (cow, encoding_used, had_error) = (String::from_utf8_lossy(expectation.as_bytes()), 1, 2);
    println!("表情文字: {:?}", expectation);
    println!("解析: {:?}", cow);
    println!("{:?}, {}", encoding_used, had_error);

    let _slice = ['l', 'o', 'r', 'e', 'm', 'z'].chunks_exact(2);
    println!("剩余: {:?}", _slice.remainder());
    _slice.for_each(|x| println!("{:?}", x));

    next(&[1, 2, 3, 4, 5, 6, 7], 2);

    next(&[User(8), User(2), User(10), User(7)], 3);

    dbg!(factorial(26));
}

fn factorial(n: u128) -> Option<u128> {
    if n <= 1 {
        return Some(1);
    }
    if let Some(a) = factorial(n - 1) {
        match n.overflowing_mul(a) {
            (v, false) => return Some(v),
            (_, true) => return None,
        }
    }

    None
}

#[derive(Debug)]
struct User(u8);

fn next<'a, T: Debug>(v: &'a [T], chunk_size: usize) -> Option<&'a [T]> {
    if v.len() < chunk_size {
        Some(dbg!(v))
    } else {
        let (chunk, remaining) = v.split_at(chunk_size);
        next(remaining, chunk_size);
        Some(dbg!(chunk))
    }
}
