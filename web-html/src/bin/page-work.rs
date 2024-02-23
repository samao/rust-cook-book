/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-23 14:08:17
 * @Last Modified by:   idzeir
 * @Last Modified time: 2024-02-23 14:08:17
 */
use reqwest::Result;
use serde::Deserialize;

fn main() -> Result<()> {
    for dep in ReverseDependencies::of("mp4")? {
        let dep = dep?;
        println!("reverse dependency: {}, {}, {}", dep.crate_id, dep.req, dep.downloads);
    }
    Ok(())
}

#[derive(Deserialize)]
struct Dependency {
    crate_id: String,
    downloads: u64,
    req: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

#[derive(Deserialize)]
struct ApiResponse {
    dependencies: Vec<Dependency>,
    meta: Meta,
}

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
            crate_id: crate_id.to_owned(),
            dependencies: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 5,
            total: 0,
        })
    }

    fn try_next(&mut self) -> Result<Option<Dependency>> {
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
            self.crate_id, self.page, self.per_page
        );
        let response = self
            .client
            .get(&url)
            .header(reqwest::header::USER_AGENT, "rust-page")
            .send()?
            .json::<ApiResponse>()?;
        println!("请求：{}", url);
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;

        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err))
        }
    }
}
