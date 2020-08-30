#[macro_use]
extern crate fff_macros;

use crate::kbd::key_event_code;
use crate::model::config::enums::BindingType;
use crate::model::config::Config;
use crate::model::file::{make, InnerFile};
use crate::model::result::Res;
use crate::ui::event::{UIEvent, UIEventSender};
use crossbeam_channel::bounded;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use std::env::current_dir;
use std::io::{stdout, Write};

mod kbd;
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

    enable_raw_mode().unwrap();

    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        Hide,
        EnableMouseCapture
    )
    .unwrap();

    let mut sender = ui::init_ui(4);

    loop {
        if let Ok(ev) = event::read() {
            match ev {
                event::Event::Key(ke) => match ke.code {
                    event::KeyCode::Char('q') => {
                        drop(sender);
                        break;
                    }
                    event::KeyCode::Char('1') => {
                        sender.send_async(UIEvent::SwitchTab(0)).unwrap();
                    }
                    event::KeyCode::Char('2') => {
                        sender.send_async(UIEvent::SwitchTab(1)).unwrap();
                    }
                    event::KeyCode::Char('3') => {
                        sender.send_async(UIEvent::SwitchTab(2)).unwrap();
                    }
                    event::KeyCode::Char('4') => {
                        sender.send_async(UIEvent::SwitchTab(3)).unwrap();
                    }
                    event::KeyCode::Char('a') => {
                        sender.loading().unwrap();
                    }
                    event::KeyCode::Char('A') => {
                        sender.send_async(UIEvent::Loading(false)).unwrap();
                    }
                    _ => {
                        continue;
                    }
                },
                event::Event::Mouse(_me) => {}
                _ => {}
            }
        } else {
            break;
        }
    }

    execute!(stdout(), DisableMouseCapture, Show, LeaveAlternateScreen).unwrap();

    disable_raw_mode().unwrap();

    Ok(())
}
