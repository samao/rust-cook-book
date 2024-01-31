use std::fmt::Display;

use bitflags::bitflags;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-31 11:02:18
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-31 11:09:59
 */
fn main() {
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    println!("FLAG_A: {}", MyFlags::FLAG_A);
    println!("FLAG_B: {}", MyFlags::FLAG_B);
    println!("FLAG_C: {}", MyFlags::FLAG_C);
    println!("FLAG_ABC: {}", MyFlags::FLAG_ABC);
    println!("e1: {}", e1);
    println!("e2: {}", e2);

    // assert_eq!((e1 | e2), MyFlags::FLAG_ABC);
    // assert_eq!((e1 & e2), MyFlags::FLAG_C);
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    struct MyFlags: u32 {
        const FLAG_A = 0b00000001;
        const FLAG_B = 0b00000010;
        const FLAG_C = 0b00000100;
        const FLAG_ABC = Self::FLAG_A.bits() | Self::FLAG_B.bits() | Self::FLAG_C.bits();
    }
}

impl MyFlags {
    pub fn clear(&mut self) -> &mut Self {
        // self.bits = 0;
        self
    }
}

impl Display for MyFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}
