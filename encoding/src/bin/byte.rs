use std::io::Error;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-06 12:26:42
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-06 12:35:15
 */
#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}
fn main() -> Result<(), Error> {
    let origin_payload = Payload { kind: u8::MAX, value: u16::MAX };
    let encoded_bytes = encode(&origin_payload)?;
    let decoded_payload = decode(&encoded_bytes)?;
    assert_eq!(dbg!(origin_payload), dbg!(decoded_payload));
    Ok(())
}

fn encode(payload: &Payload) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];

    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;

    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload, Error> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };

    Ok(payload)
}
