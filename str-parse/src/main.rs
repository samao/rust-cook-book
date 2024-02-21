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
    let color = r"#fa7268";

    match RGB::from_str(color) {
        Ok(rgb @ RGB { r, g, b }) => {
            println!(
                "The RGB color is {:?} -> code is R: {}, G: {}, B: {}",
                rgb, r, g, b
            );
        }
        _ => {}
    }

    assert_eq!(
        RGB::from_str("#ff00ff").unwrap(),
        RGB {
            r: 255,
            g: 0,
            b: 255
        }
    );
}

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = u8::from_str_radix(&s[1..3], 16)?;
        let g = u8::from_str_radix(&s[3..5], 16)?;
        let b = u8::from_str_radix(&s[5..7], 16)?;
        Ok(RGB { r, g, b })
    }
}
