use std::{num::ParseIntError, str::FromStr};

use unicode_segmentation::UnicodeSegmentation;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-21 16:52:32
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-21 16:53:04
 */
fn main() {
    let name = "José Guimarães\r\n";

    let graphemes = UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();

    assert_eq!(graphemes[3], "é");

    println!("{:?}", graphemes);

    impl_fromstr();
}

fn impl_fromstr() {
    let color = r"#fa7268cc";

    match RGBA::from_str(color) {
        Ok(rgb @ RGBA { r, g, b, a }) => {
            println!(
                "The RGBA color is {:?} -> code is R: {}, G: {}, B: {}, A: {}",
                rgb, r, g, b, a
            );
        }
        _ => {}
    }

    assert_eq!(
        RGBA::from_str("#ff00ff01").unwrap(),
        RGBA {
            r: 255,
            g: 0,
            b: 255,
            a: 1,
        }
    );

    println!("{:?}", RGBA::from_str("#aabbccdd"));
}

#[derive(Debug, PartialEq)]
struct RGBA {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGBA {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = u8::from_str_radix(&s[1..3], 16)?;
        let g = u8::from_str_radix(&s[3..5], 16)?;
        let b = u8::from_str_radix(&s[5..7], 16)?;
        let a = u8::from_str_radix(&s[7..9], 16)?;
        Ok(RGBA { r, g, b, a })
    }
}
