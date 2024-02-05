/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-05 10:49:21
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-05 10:50:15
 */
extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}
fn main() {
    unsafe { println!("multiply: {}", multiply(5, 7)) }
}
