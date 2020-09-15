#[macro_use]
extern crate fff_macros;

use crate::action::init_action;
use crate::config::Config;
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
use std::sync::Arc;

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
    let c = Arc::new(Config::new(&home));

    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).unwrap();

    std::panic::set_hook(Box::new(|info| {
        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        log::error!("panic-ed error: {}", info);
        println!("{}", info);
    }));

    let sender = ui::init_ui(4);
    let (k, ac) = kbd::init_kbd(c.clone(), sender.clone());
    let kbd = Arc::new(k);
    let mut ws = Workspace::new(wd, home, sender.clone(), kbd.clone());
    ws.init().await.unwrap();
    ws.switch_to(0).await.unwrap();

    tokio::select! {
        _ = init_action(ac, ws, sender.clone(), kbd.clone()) => {},
        _ = kbd.start() => {}
    }

    execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();

    Ok(())
}
