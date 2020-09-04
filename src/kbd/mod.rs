use crate::config::Config;
use crate::kbd::mode::KeyEventHandler;
use crate::kbd::normal_mode::NormalMode;
use crate::ui::event::UIEventSender;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::event::{read, Event, KeyEvent};

use std::sync::{Arc, Mutex};

mod code;
mod input_mode;
mod mode;
mod normal_mode;

enum ModeEnum {
    Normal(NormalMode),
}

impl ModeEnum {
    fn handle(&mut self, ev: KeyEvent) {
        match self {
            ModeEnum::Normal(n) => n.mode_mut().handle(ev),
        }
    }

    fn name(&mut self) -> String {
        match self {
            ModeEnum::Normal(n) => n.mode_mut().name.clone(),
        }
    }

    fn is_quit(&self, ev: &KeyEvent) -> bool {
        match self {
            ModeEnum::Normal(n) => n.mode().is_quit(ev),
            _ => false,
        }
    }
}

pub struct Kbd {
    mode: Arc<Mutex<ModeEnum>>,
    ui_event: UIEventSender,
    sender: Sender<String>,
}

impl Kbd {
    pub async fn start(&self) -> i32 {
        let mode = self.mode.clone();
        tokio::spawn(async move {
            loop {
                match read() {
                    Ok(ev) => {
                        let mut lock = mode.lock().unwrap();
                        match ev {
                            Event::Key(ke) if lock.is_quit(&ke) => break 1,
                            Event::Key(ke) => lock.handle(ke),
                            _ => (),
                        };
                    }
                    _ => break 0,
                }
            }
        })
        .await
        .unwrap()
    }
}

pub struct ActionReceiver(Receiver<String>);

pub fn init_kbd(config: &Config, ui_event: UIEventSender) -> (Kbd, ActionReceiver) {
    let (tx, rx) = bounded(10);
    let ar = ActionReceiver(rx);
    let mode = Arc::new(Mutex::new(ModeEnum::Normal(NormalMode::new(
        &config,
        tx.clone(),
        ui_event.clone(),
    ))));

    let kbd = Kbd {
        mode: mode.clone(),
        ui_event,
        sender: tx,
    };

    (kbd, ar)
}
