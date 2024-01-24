/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-01-24 17:13:06
 * @Last Modified by:   idzeir
 * @Last Modified time: 2024-01-24 17:13:06
 */
pub mod error;

pub type Result<T> = core::result::Result<T, error::MYError>;
