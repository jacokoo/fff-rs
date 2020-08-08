#[macro_use]
extern crate fff_macros;

use crate::model::config::enums::BindingType;
use crate::model::config::Config;
use crate::model::file::{make, InnerFile};
use crate::model::result::Res;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{event, execute};
use std::env::current_dir;
use std::io::{stdout, Write};

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
    println!("{}", "什么东西啊".len());
    println!("{}", "hello".len());

    "什么东西啊💣"
        .chars()
        .for_each(|it| println!("{}", it.len_utf8()));

    enable_raw_mode().unwrap();

    execute!(stdout(), Clear(ClearType::All), Hide, EnableMouseCapture);

    ui::demo();
    stdout().flush();

    loop {
        if let Ok(ev) = event::read() {
            if let event::Event::Key(ke) = ev {
                match ke.code {
                    event::KeyCode::Char('q') => break,
                    _ => continue,
                }
            }
        } else {
            break;
        }
    }

    execute!(stdout(), Show, DisableMouseCapture);

    disable_raw_mode().unwrap();

    Ok(())
}
