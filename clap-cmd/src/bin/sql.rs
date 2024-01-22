use rusqlite::{params, Connection};
use std::{collections::HashMap, fmt::Display};

fn main() {
    if let Ok(conn) = Connection::open("cats.db") {
        if let Err(reason) = conn.execute(
            "create table if not exists cat_colors (
            id integer primary key,
            name text not null unique
        )",
            params![],
        ) {
            panic!("数据库table创建失败：{}", reason);
        };
        if let Err(reason) = conn.execute(
            "create table if not exists cats (
            id integer primary key,
            name text not null,
            color_id integer not null references cat_colors(id)
        )",
            params![],
        ) {
            panic!("数据库table创建失败：{}", reason);
        }

        let cat_colors = HashMap::from([
            (String::from("Blue"), vec!["Tigger", "sammy"]),
            (String::from("Black"), vec!["Oreo", "Biscuit"]),
        ]);

        for (color, catnames) in &cat_colors {
            if let Err(result) =
                conn.execute("INSERT INTO cat_colors (name) values (?1)", &[&color])
            {
                println!("插入失败: {}", result);
            }
            let last_id = conn.last_insert_rowid().to_string();
            for cat in catnames {
                if let Err(reason) = conn.execute(
                    "INSERT INTO cats (name, color_id) values (?1, ?2)",
                    &[&cat.to_string(), &last_id],
                ) {
                    println!("插入数据有异: {}", reason);
                }
            }
        }

        if let Ok(mut stmt) = conn.prepare(
            "SELECT c.name, cc.name from cats c
                INNER JOIN cat_colors cc
                ON cc.id = c.color_id;",
        ) {
            if let Ok(cats) = stmt.query_map(params![], |row| {
                Ok(Cat {
                    name: row.get(0).unwrap(),
                    color: row.get(1).unwrap(),
                })
            }) {
                for cat in cats {
                    println!("Found cat {:?}", cat);
                }
            } else {
                println!("query map error")
            }
        } else {
            println!("select error");
        }
    } else {
        println!("无法打开数据库文件或已经损坏");
    }
    if let Err(msg) = connect() {
        println!("DOG DB is: {}", msg);
    }
}

fn connect() -> Result<(), IoError> {
    let conn = Connection::open("dogs.db")?;
    conn.execute(
        "create table if not exists dog_colors (
            id integer primary key,
            name text not null unique
        )",
        params![],
    )?;
    conn.execute(
        "create table if not exists dogs (
            id integer primary key,
            name text not null,
            color_id integer not null references dog_colors(id)
        )",
        params![],
    )?;

    let dogs = HashMap::from([
        (String::from("Yellow"), vec!["Puppy", "Stone"]),
        (String::from("Prink"), vec!["Doggy", "Piggy"]),
    ]);

    for (color, dogname) in &dogs {
        conn.execute("INSERT INTO dog_colors (name) values (?1)", params![color])?;

        let last_id = conn.last_insert_rowid().to_string();

        for cat in dogname {
            conn.execute(
                "INSERT INTO dogs (name, color_id) values (?1, ?2)",
                params![&cat, &last_id],
            )?;
        }
    }

    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from dogs c INNER JOIN dog_colors cc ON cc.id = c.color_id",
    )?;

    let dogs = stmt.query_map(params![], |row| {
        Ok(Dog {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for dog in dogs {
        println!("Found Dog {:?}", dog);
    }
    Ok(())
}

#[derive(Debug)]
struct IoError(String);

impl std::error::Error for IoError {}
impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Io Error: {}", self.0)
    }
}

impl From<rusqlite::Error> for IoError {
    fn from(value: rusqlite::Error) -> Self {
        IoError(value.to_string())
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
    color: String,
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}
