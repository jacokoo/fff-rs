use crate::config::Config;
use crate::kbd::input_mode::InputMode;
use crate::kbd::mode::KeyEventHandler;
use crate::kbd::normal_mode::NormalMode;
use crate::ui::event::UIEventSender;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::event::{read, Event, KeyEvent};
use std::borrow::Borrow;

use std::sync::{Arc, Mutex};

mod code;
mod input_mode;
mod mode;
mod normal_mode;

pub enum Answer {
    Yes,
    No,
    YesToAll,
    NoToAll,
}

impl Answer {
    fn try_from(s: &str, multiple: bool) -> Option<Answer> {
        match s {
            "y" => Some(Answer::Yes),
            "n" => Some(Answer::No),
            "Y" if multiple => Some(Answer::YesToAll),
            "N" if multiple => Some(Answer::NoToAll),
            _ => None,
        }
    }

    fn desc(multiple: bool) -> Vec<String> {
        if multiple {
            vec![
                "yes".to_string(),
                "no".to_string(),
                "Yes to all".to_string(),
                "No to All".to_string(),
            ]
        } else {
            vec!["yes".to_string(), "no".to_string()]
        }
    }
}

enum ModeEnum {
    Normal(NormalMode),
    Input(InputMode),
}

impl ModeEnum {
    fn handle(&mut self, ev: KeyEvent) {
        match self {
            ModeEnum::Normal(n) => n.handle(ev),
            ModeEnum::Input(n) => n.handle(ev),
        }
    }

    fn name(&mut self) -> String {
        match self {
            ModeEnum::Normal(n) => n.name(),
            ModeEnum::Input(n) => n.name(),
        }
    }

    fn is_quit(&self, ev: &KeyEvent) -> bool {
        match self {
            ModeEnum::Normal(n) => n.is_quit(ev),
            _ => false,
        }
    }
}

pub struct Kbd {
    mode: Arc<Mutex<ModeEnum>>,
    config: Arc<Config>,
    ui_event: UIEventSender,
    sender: Sender<String>,
}

impl Kbd {
    pub async fn start(&self) -> i32 {
        let mode = self.mode.clone();
        let s = self.sender.clone();
        tokio::spawn(async move {
            loop {
                match read() {
                    Ok(ev) => {
                        let mut lock = mode.lock().unwrap();
                        match ev {
                            Event::Key(ke) if lock.is_quit(&ke) => {
                                s.send("Quit".to_string()).unwrap();
                                break 1;
                            }
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

    pub async fn request_input(&self, _prompt: &str) -> Option<String> {
        let (mode, rx) = InputMode::new_input(
            self.config.borrow(),
            self.sender.clone(),
            self.ui_event.clone(),
        );
        self.set_mode(ModeEnum::Input(mode));
        tokio::spawn(async move { rx.recv().unwrap() })
            .await
            .unwrap()
    }

    pub async fn request_answer(&self, _prompt: &str, multiple: bool) -> Option<Answer> {
        let (mode, rx) = InputMode::new_answer(
            self.config.borrow(),
            self.sender.clone(),
            self.ui_event.clone(),
            multiple,
        );
        self.set_mode(ModeEnum::Input(mode));
        tokio::spawn(async move { rx.recv().unwrap() })
            .await
            .unwrap()
    }

    pub fn switch_to_normal(&self) {
        self.set_mode(ModeEnum::Normal(NormalMode::new(
            self.config.borrow(),
            self.sender.clone(),
            self.ui_event.clone(),
        )));
    }

    fn set_mode(&self, mode: ModeEnum) {
        let mut s = self.mode.lock().unwrap();
        *s = mode;
    }
}

pub struct ActionReceiver(pub Receiver<String>);

pub fn init_kbd(config: Arc<Config>, ui_event: UIEventSender) -> (Kbd, ActionReceiver) {
    let (tx, rx) = bounded(10);
    let ar = ActionReceiver(rx);
    let mode = Arc::new(Mutex::new(ModeEnum::Normal(NormalMode::new(
        config.borrow(),
        tx.clone(),
        ui_event.clone(),
    ))));

    let kbd = Kbd {
        mode: mode.clone(),
        config,
        ui_event,
        sender: tx,
    };

    (kbd, ar)
}
