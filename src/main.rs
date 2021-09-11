use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;

#[derive(Debug)]
struct Database {
    data: HashMap<String, String>,
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
            return Ok(Database { data })
        }
        Ok(Database { data: HashMap::<String, String>::new() })
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_lowercase(), value.to_lowercase());
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (k, v) in self.data {
            contents.push_str(format!("{}\t{}\n", k, v).as_str());
        }
        fs::write("kv.db", contents)
    }

}

fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().expect("missing key");
    let value = args.next().expect("missing value");
    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(&key, &value);
    database.flush().expect("error writing database");
}
