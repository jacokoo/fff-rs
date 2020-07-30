#[macro_use]
pub mod enums;

use crate::model::config::enums::{BindingType, ColorType, ColorValue};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug)]
pub enum Action {
    Normal(String),
    Prefixed(HashMap<String, String>),
}

pub type Bindings = HashMap<String, Action>;

#[derive(Debug)]
pub struct Config {
    pub bindings: HashMap<BindingType, Bindings>,
    pub color: HashMap<ColorType, ColorValue>,
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

    pub fn get_action(&self, bt: &BindingType, key: &str) -> Option<&Action> {
        let bs = self.bindings.get(bt).unwrap();
        if bs.contains_key(key) {
            return bs.get(key);
        }

        let bs = self.bindings.get(&BindingType::All).unwrap();
        return bs.get(key);
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
            let vv = ColorValue::from(read_str(v, "color").borrow());
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
                    if let Some(Action::Normal(_)) = map.get(k) {
                        map.insert(k.clone(), Action::Prefixed(HashMap::new()));
                    }
                    let ac = map
                        .entry(k.to_owned())
                        .or_insert_with(|| Action::Prefixed(HashMap::new()));
                    if let Action::Prefixed(mm) = ac {
                        for (kk, vv) in tt.iter() {
                            mm.insert(kk.to_owned(), read_str(vv, kk.borrow()));
                        }
                    }
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
