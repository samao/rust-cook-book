use std::error::Error;

use serde_json::Value;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2024-02-05 16:14:53
 * @Last Modified by: idzeir
 * @Last Modified time: 2024-02-05 16:59:00
 */
fn main() -> Result<(), Box<dyn Error>> {
    let a = r#"{
        "userid": 103609,
        "verified": true,
        "access_privileges": [
          "user",
          "admin"
        ]
      }"#;

    let b = format!(
        r#"{{
        "userid": {},
        "verified": {},
        "access_privileges": [
          "user",
          "admin"
        ]
      }}"#,
        203609, false
    );

    let b = b.as_str();

    assert_ne!(a, b);
    let parsed: Value = serde_json::from_str(b)?;
    // let userid = match &parsed["userid"] {
    //     Value::Number(num) => match num.as_u64() {
    //         Some(num) => num,
    //         None => unreachable!(),
    //     },
    //     _ => unreachable!(),
    // };
    // println!("{:?}", userid);
    println!(
        "userid: {}, verified: {}, access_privileges: {}",
        parsed["userid"], parsed["verified"], parsed["access_privileges"]
    );

    // assert_eq!(parsed["userid"], 203609);

    if parsed["verified"] == false {
        println!("用户验证未通过");
    }

    match &parsed["userid"].as_u64() {
        Some(1..=100) => {
            println!("内部id号，不能外泄");
        }
        Some(num @ 1..=1000000) => {
            println!("老用户id: {}", num);
        }
        Some(num) => {
            println!("userid is: {}", num);
        }
        _ => unreachable!(),
    }
    Ok(())
}
