use std::{
    borrow::Cow,
    collections::HashSet,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::lazy_static;
use regex::{Regex, RegexSetBuilder};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-21 15:05:38
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-21 15:06:07
 */
fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );

    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);

    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_tag(tweet);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert!(!tags.contains("#2") && !tags.contains("#_"));
    assert_eq!(3, tags.len());

    extract_phone()?;
    mutil_reg()?;

    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);

    println!("{} -> {}", before, after);

    Ok(())
}

// 转时间格式
fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX: Regex =
            Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

fn mutil_reg() -> Result<(), Box<dyn Error>> {
    let log_path = "application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\:443"#,
        r#"warning.*timeout expired"#,
    ])
    .case_insensitive(true)
    .build()?;

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));
    Ok(())
}

struct PhoneNumber<'a> {
    area: &'a str,
    exchange: &'a str,
    subscriber: &'a str,
}

impl<'a> Display for PhoneNumber<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
    }
}

fn extract_phone() -> Result<(), Box<dyn Error>> {
    let phone_text = "
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020";

    let re = Regex::new(
        r#"(?x)
        (?:\+?1)?                       #国家代码
        [\s\.]?
        (([2-9]\d{2})|\(([2-9]\d{2})\))  #地区代码
        [\s\.\-]?
        ([2-9]\d{2})                    #交换代码
        [\s\.\-]?
        (\d{4})                         #用户号码"#,
    )?;

    let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
        let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
        match groups {
            (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                area: area.as_str(),
                exchange: ext.as_str(),
                subscriber: sub.as_str(),
            }),
            _ => None,
        }
    });
    phone_numbers.for_each(|p| println!("☎️: {}", p));

    // assert_eq!(
    //     phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
    //     vec![
    //         "1 (505) 881-9292",
    //         "1 (505) 778-2212",
    //         "1 (505) 881-9297",
    //         "1 (202) 991-9534",
    //         "1 (555) 392-0011",
    //         "1 (800) 233-2010",
    //         "1 (299) 339-1020",
    //     ]
    // );

    Ok(())
}

fn extract_login(input: &str) -> Option<&str> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
        ^(?P<login>[^@\s]+)@
        ([[:word:]]+\.)*
        [[:word:]]+$
        ").unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

fn extract_tag(text: &str) -> HashSet<&str> {
    lazy_static::lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
    }

    HASHTAG_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}
