#[macro_use]
extern crate fff_macros;

use crate::config::Config;
use crate::model::file::{make, InnerFile};
use crate::model::result::Res;

use crossterm::cursor::{Hide, Show};

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::env::current_dir;
use std::io::{stdout, Write};

#[macro_use]
mod config;

mod action;
mod common;
mod kbd;
mod model;
mod ui;

#[tokio::main]
async fn main() -> Res<()> {
    let wd = current_dir()?;
    let home = dirs::home_dir().unwrap();

    if let InnerFile::Dir(dir) = make(&wd)? {
        for item in dir.list().await? {
            let info = item.info();
            println!("{}, {}, {}", info.name, info.path, info.mode);
            if let Some(link) = &info.link {
                println!("is link {}, {}", link.target, link.broken)
            }
        }
    }

    let c = Config::new(&home);

    enable_raw_mode().unwrap();

    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).unwrap();

    let sender = ui::init_ui(4);
    let (kbd, _ac) = kbd::init_kbd(&c, sender.clone());
    kbd.start().await;

    execute!(stdout(), Show, LeaveAlternateScreen).unwrap();

    disable_raw_mode().unwrap();

    Ok(())
}
