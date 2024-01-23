use std::fmt::Display;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-23 14:53:21
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-01-23 15:27:22
 */
#[derive(Debug)]
pub struct MYError(String);

impl std::error::Error for MYError {}
impl Display for MYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MYERROR: {}", self.0)
    }
}

impl From<std::io::Error> for MYError {
    fn from(value: std::io::Error) -> Self {
        MYError(value.to_string())
    }
}

impl From<ring::error::Unspecified> for MYError {
    fn from(value: ring::error::Unspecified) -> Self {
        MYError(value.to_string())
    }
}
