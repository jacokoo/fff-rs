use std::env::{current_dir};
use std::io::Result;
use crate::model::file::{FileType, make};
use crate::model::config::Config;
use crate::model::config::enums::BindingType;

mod ui;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let wd = current_dir()?;

    if let FileType::Dir(dir) = make(&wd)? {
        for item in dir.as_ref().list().await? {
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