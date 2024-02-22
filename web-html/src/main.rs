use std::{borrow::Cow, collections::HashSet};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-22 12:16:43
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-22 12:33:29
 */
use error_chain::error_chain;
use regex::Regex;
use reqwest::StatusCode;
use select::{document::Document, predicate::Name};
use url::{Position, Url};

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        UrlParseError(url::ParseError);
        JoinError(tokio::task::JoinError);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("http://eid.csrc.gov.cn/fund/disclose/fund_detail_search.do?cFundCode=12620&rnd=0.8809817585748807")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;

    let doc = Document::from(res.as_str());
    println!("获取页面链接**********");
    doc.find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    println!("\r\n检测页面死链接**********");
    let base_url = get_base_url(&url, &doc).await?;
    let base_parser = Url::options().base_url(Some(&base_url));

    let links = doc
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|link| !link.starts_with("javascript:"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect::<HashSet<_>>();

    let mut tasks = vec![];

    for link in links {
        tasks.push(tokio::spawn(async move {
            if let Ok(_) = check_link(&link).await {
                println!("{} is OK", link);
            } else {
                println!("{} is Broken", link);
            }
        }));
    }

    for task in tasks {
        task.await?
    }

    println!("\r\n提取页面唯一性链接**********");
    let content = reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )
    .await?
    .text()
    .await?;

    println!("{:#?}", extract_links(content.as_str()));
    Ok(())
}

async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

fn extract_links(content: &str) -> HashSet<Cow<str>> {
    lazy_static::lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(
        r"(?x)
    \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]     # internal links
    |
    (url=|URL\||\[)(?P<external>http.*?)[ \|}]  # external links
    ",
    )
    .unwrap();
    }

    let links = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect::<HashSet<_>>();

    links
}
