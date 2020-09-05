#[macro_use]
extern crate fff_macros;

use crate::action::init_action;
use crate::config::Config;
use crate::model::file::{make, InnerFile};
use crate::model::init_state;
use crate::model::result::Res;
use crate::model::state::workspace::Workspace;
use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use simplelog::{LevelFilter, WriteLogger};
use std::env::current_dir;
use std::fs::File;
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
    WriteLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("log.log").unwrap(),
    )
    .unwrap();

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

    std::panic::set_hook(Box::new(|info| {
        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        log::error!("panic-ed error: {}", info);
        println!("{}", info);
    }));

    let sender = ui::init_ui(4);
    let (kbd, ac) = kbd::init_kbd(&c, sender.clone());
    let mut ws = Workspace::new(wd, home, sender.clone());
    ws.init().await.unwrap();
    ws.switch_to(0).await.unwrap();

    tokio::select! {
        _ = init_action(ac, ws) => {},
        _ = kbd.start() => {}
    }

    execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();

    Ok(())
}
