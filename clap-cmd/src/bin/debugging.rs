use log::{Level, SetLoggerError};

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
fn main() {
    // 1.自定义logger
    if let Err(err) = console_init() {
        println!("日志启动失败: {}", err);
    }

    //2. env_logger::init();
    // 下面保留error
    //3. Builder::new().target(Target::Stdout).init();
    execute_query("Drop Table Students");

    if let Err(err) = execute_query_error("Drop Tbody students.") {
        log::error!("Failed to execute query: {}", err);
    }
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
