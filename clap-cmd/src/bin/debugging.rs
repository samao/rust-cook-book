use chrono::Local;
use env_logger::Builder;
use log::{Level, SetLoggerError};
use std::io::Write;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-31 17:32:50
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-31 17:34:14
 */
fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

//RUST_LOG=debug cargo run --bin debugging
fn main() -> clap_cmd::Result<()> {
    // 1.自定义logger
    // console_init()?;

    //2. 默认
    // env_logger::init();
    // 下面保留error
    //3. Builder::new().target(Target::Stdout).init();
    //4. 自定义环境变量
    parse_from_env();

    execute_query("Drop Table Students");

    // if let Err(err) = execute_query_error("Drop Tbody students.") {
    //     log::error!("Failed to execute query: {}", err);
    // }
    log_test::run_say_hi();
    log_test::bar::say_module_hi();

    execute_query_error("Drop Tbody students.")?;

    Ok(())
}

fn parse_from_env() {
    // let level = env::var("RUST_LOG").unwrap_or_default();
    // println!("QIE_LOG: {}", level);
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Off)
        .parse_env("QIE_LOG")
        .init();
    log::debug!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "##message");
}

fn console_init() -> Result<(), SetLoggerError> {
    log::set_logger(&ConsoleLogger)?;
    log::set_max_level(log::LevelFilter::Info);

    log::info!("hello log");
    log::warn!("warning");
    log::error!("oops!!!");
    Ok(())
}

fn execute_query_error(query: &str) -> Result<(), Box<String>> {
    Err(Box::new(format!("I'm afraid I can't do that. <{}>", query)))
}

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

mod log_test {
    pub mod bar {
        pub fn say_module_hi() {
            log::debug!("i am log test say hi");
            log::warn!("say_module_hi warning");
            log::error!("say_module_hi oops!!!");

            log::info!("奇偶数 -> {:?}", is_odd(-31));
        }

        fn is_odd(value: isize) -> (isize, bool) {
            (value, value % 2 == 0)
        }
    }

    pub fn run_say_hi() {
        log::debug!("-i am log test say hi");
        log::warn!("-say_module_hi warning");
        log::error!("-say_module_hi oops!!!");
    }
}
