use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Write};
use std::path::PathBuf;

pub struct Bookmark {
    path: PathBuf,
    items: HashMap<String, String>,
}

impl Bookmark {
    pub fn new(home: &PathBuf) -> Bookmark {
        let p = home.join(".config/fff/bookmarks");
        if !p.exists() {
            File::create(&p).unwrap();
        }
        let s = read_to_string(&p).unwrap();
        let mut items = HashMap::new();
        s.split("\n").for_each(|line| {
            let s: Vec<_> = line.split("=").collect();
            items.insert(s[0].to_string(), s[1].to_string());
        });
        Bookmark { path: p, items }
    }

    pub fn add(&mut self, name: String, path: String) {
        self.items.insert(name, path);
        self.write();
    }

    pub fn del(&mut self, name: String) {
        if let Some(_) = self.items.remove(&name) {
            self.write();
        }
    }

    pub fn items(&self) -> &HashMap<String, String> {
        &self.items
    }

    fn write(&self) {
        let mut writer = OpenOptions::new().write(true).open(&self.path).unwrap();
        self.items.iter().for_each(|(k, v)| {
            write!(writer, "{}={}\n", k, v).unwrap();
        });
    }
}
