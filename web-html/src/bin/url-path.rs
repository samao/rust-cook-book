/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-22 14:45:58
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-22 14:52:50
 */
use error_chain::error_chain;
use url::{Host, Origin, Position, Url};

error_chain! {
    foreign_links {
        ParseUrlError(url::ParseError);
    }
}
fn main() -> Result<()> {
    let s = "wss://www.github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());
    println!("The host part of the URL is: {:?}", parsed.host());
    println!(
        "The port part of the URL is: {:?} = {:?}",
        parsed.port().or_else(|| match parsed.scheme() {
            "https" => Some(443),
            "http" => Some(80),
            "ws" => Some(666),
            "wss" => Some(555),
            _ => None,
        }),
        parsed.port_or_known_default()
    );

    let base = base_url(parsed)?;
    assert_eq!(base.as_str(), "wss://www.github.com/");
    println!("The base of the URL is: {}", base);

    let path = "/rust-lang/cargo";
    let gh = build_github_url(path)?;
    println!("The joined URL is: {}", gh);

    let s = "ftp://rust-lang.org/example";
    let url = Url::parse(s)?;
    let expected_scheme = "wss".to_owned();
    let expected_host = Host::Domain("live.qq.com".to_owned());
    let expected_port = 443;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    println!("url is: {:?}. expected is: {:?}", url.origin(), expected);

    remove_hash_and_search()?;

    let mut url = Url::parse("https://live.qq.com/news/detail/19293021?from=homepage#bottom")?;
    if let Ok(mut path) = url.path_segments_mut() {
        path.clear();
    }

    println!("cleaned if let is {}", url);
    Ok(())
}

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => return Err(Error::from_kind(ErrorKind::Msg("CannotBeABase".to_owned()))),
    }
    url.set_query(None);
    Ok(url)
}

fn build_github_url(path: &str) -> Result<Url> {
    const GITHUB: &'static str = "https://github.com";
    let base = Url::parse(GITHUB).expect("hardcoded URL is known tobe valid");
    let joined = base.join(path)?;

    Ok(joined)
}

fn remove_hash_and_search() -> Result<()> {
    let parsed =
        Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open#total")?;
    let cleaned = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
