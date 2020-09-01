#[macro_use]
pub mod enums;

use crate::config::enums::{BindingType, ColorType};
use crossterm::style::Color;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use toml::Value;

#[derive(Debug, Clone)]
pub enum Action {
    Normal(String),
    Prefixed(HashMap<String, String>),
}

pub type Bindings = HashMap<String, Action>;

#[derive(Debug)]
pub struct Config {
    bindings: HashMap<BindingType, Bindings>,
    pub color: HashMap<ColorType, Color>,
    pub editor: String,
    pub shell: String,
    pub pager: String,
}

impl Config {
    pub fn new(home: PathBuf) -> Config {
        let mut c = Config {
            bindings: HashMap::new(),
            color: HashMap::new(),
            editor: "".to_string(),
            shell: "".to_string(),
            pager: "".to_string(),
        };

        read(&mut c, &DEFAULT);
        let file = home.join(".config/fff/config.toml");
        if file.exists() {
            read(&mut c, &std::fs::read_to_string(file).unwrap());
        }

        if c.shell == "" {
            if let Ok(s) = std::env::var("SHELL") {
                c.shell = s;
            } else {
                c.shell = "bash".to_string();
            }
        }

        return c;
    }

    pub fn bindings(&self, bt: &BindingType) -> Bindings {
        let mut bs: Bindings = self
            .bindings
            .get(&BindingType::All)
            .map_or_else(|| HashMap::new(), |v| v.clone());

        if let Some(v) = self.bindings.get(bt) {
            v.iter().for_each(|(k, vv)| {
                bs.insert(k.clone(), vv.clone());
            });
        }

        bs
    }
}

const DEFAULT: &'static str = include_str!("config.toml");

fn read_str(value: &Value, message: &str) -> String {
    if let Value::String(ss) = value {
        return ss.to_string();
    }
    panic!("{} is not a string", message);
}

fn read_color(config: &mut Config, value: &Value) {
    if let Value::Table(table) = value {
        for (k, v) in table.iter() {
            let kk = ColorType::try_from(k.borrow()).unwrap();
            let vv = Color::try_from(read_str(v, "color").borrow()).unwrap();
            config.color.insert(kk, vv);
        }
        return;
    }
    panic!("color is not a table");
}

fn read_binding(config: &mut Config, value: &Value) {
    if let Value::Table(table) = value {
        for (k, v) in table.iter() {
            let kk = BindingType::try_from(k.borrow()).unwrap();
            let bd = config.bindings.entry(kk).or_insert_with(|| HashMap::new());
            read_binding_type(bd, v, k.borrow());
        }
        return;
    }
    panic!("binding is not a table");
}

fn read_binding_type(map: &mut Bindings, value: &Value, key: &str) {
    if let Value::Table(table) = value {
        for (k, v) in table.iter() {
            match v {
                Value::String(v) => {
                    map.insert(k.to_owned(), Action::Normal(v.to_owned()));
                }
                Value::Table(tt) => {
                    let mut mp: HashMap<String, String> = HashMap::new();
                    for (kk, vv) in tt.iter() {
                        mp.insert(kk.to_owned(), read_str(vv, kk.borrow()));
                    }
                    map.insert(k.to_owned(), Action::Prefixed(mp));
                }
                _ => panic!("in valid action binding"),
            }
        }
        return;
    }
    panic!("{} binding is not a table", key);
}

fn read(config: &mut Config, content: &str) {
    let v: Value = toml::from_str(content).unwrap();
    if let Value::Table(table) = v {
        if let Some(p) = table.get("pager") {
            config.pager = read_str(p, "pager");
        }

        if let Some(p) = table.get("shell") {
            config.shell = read_str(p, "shell");
        }

        if let Some(p) = table.get("editor") {
            config.editor = read_str(p, "editor")
        }

        if let Some(p) = table.get("color") {
            read_color(config, p);
        }

        if let Some(p) = table.get("binding") {
            read_binding(config, p);
        }
        return;
    }
    panic!("can not parse config file")
}
