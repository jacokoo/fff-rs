#[macro_use]
extern crate fff_macros;

use crate::model::config::enums::BindingType;
use crate::model::config::Config;
use crate::model::file::{make, InnerFile};
use crate::model::result::Res;
use std::env::current_dir;

mod model;
mod ui;

#[tokio::main]
async fn main() -> Res<()> {
    let wd = current_dir()?;

    if let InnerFile::Dir(dir) = make(&wd)? {
        for item in dir.list().await? {
            let info = item.info();
            println!("{}, {}, {}", info.name, info.path, info.mode);
            if let Some(link) = &info.link {
                println!("is link {}, {}", link.target, link.broken)
            }
        }
    }

    let c = Config::new(dirs::home_dir().unwrap());
    println!("{:?}", c.get_action(&BindingType::Normal, "ctrl-q"));

    Ok(())
}
