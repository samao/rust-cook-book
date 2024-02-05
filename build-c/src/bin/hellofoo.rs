/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-05 14:31:15
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-05 14:31:36
 */
extern "C" {
    fn print_app_info();
}

fn main() {
    unsafe { print_app_info() }
}
