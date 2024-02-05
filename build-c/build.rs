/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-05 10:08:10
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-05 14:43:33
 */

use std::env::VarError;

fn main() {
    println!("cargo:rerun-if-env-changed=BUILD");
    match std::env::var("BUILD").or_else(|_| Ok::<String, VarError>("ALL".to_owned())) {
        Ok(build_type) => match build_type.as_str() {
            "C" => {
                build_c();
            }
            "C++" => {
                build_c_plus();
            }
            "C#" => {
                build_c_defined();
            }
            "ALL" => {
                build_c();
                build_c_plus();
                build_c_defined();
            }
            _ => {}
        },
        _ => {}
    }
}

fn build_c() {
    println!("cargo:rerun-if-changed=src/hello.c");
    cc::Build::new().file("src/hello.c").compile("helloc"); //输出libhello.a
}

fn build_c_plus() {
    println!("cargo:rerun-if-changed=src/hello.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/hello.cpp")
        .compile("hellocpp");
}

fn build_c_defined() {
    println!("cargo:rerun-if-changed=src/hello_foo.c");
    cc::Build::new()
        .define("APP_NAME", "\"QIE_foo\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOM", None)
        .file("src/hello_foo.c")
        .compile("hellofooc")
}
