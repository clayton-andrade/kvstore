use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;

#[derive(Debug)]
struct Database {
    data: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        if path::Path::new("kv.db").exists() {
            let contents = fs::read_to_string("kv.db")?;
            let mut data = HashMap::new();
            for line in contents.lines() {
                let (key, value) = line.split_once('\t').expect("Corrupt database");
                data.insert(key.to_owned(), value.to_owned());
            }
            return Ok(Database { data, flush: false });
        }
        Ok(Database {
            data: HashMap::new(),
            flush: false,
        })
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_lowercase(), value.to_lowercase());
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush = true;
        let mut contents = String::new();
        for (k, v) in &self.data {
            contents.push_str(k);
            contents.push('\t');
            contents.push_str(v);
            contents.push('\n');
        }
        fs::write("kv.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            self.flush().expect("error writing database");
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().expect("missing key");
    let value = args.next().expect("missing value");
    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(&key, &value);
    // database.flush().expect("error writing database");
    // database.insert("hello", "world");
}
