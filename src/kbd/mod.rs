use crate::config::Config;
use crate::kbd::mode::KeyEventHandler;
use crate::kbd::normal_mode::NormalMode;
use crate::ui::event::UIEventSender;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::event::{read, Event, KeyEvent};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

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
}

pub struct Kbd {
    mode: Arc<Mutex<ModeEnum>>,
    uiEvent: UIEventSender,
    sender: Sender<String>,
}

impl Kbd {
    pub async fn start(&self) {
        let mode = self.mode.clone();
        tokio::spawn(async move {
            while let Ok(ev) = read() {
                let mut lock = mode.lock().unwrap();
                match ev {
                    Event::Key(ke) => lock.handle(ke),
                    _ => (),
                }
            }
        })
        .await
        .unwrap();
    }
}

pub struct ActionReceiver(Receiver<String>);

pub fn init_kbd(config: &Config, uiEvent: UIEventSender) -> (Kbd, ActionReceiver) {
    let (tx, rx) = bounded(10);
    let ar = ActionReceiver(rx);
    let mode = Arc::new(Mutex::new(ModeEnum::Normal(NormalMode::new(
        &config,
        tx.clone(),
        uiEvent.clone(),
    ))));

    let kbd = Kbd {
        mode: mode.clone(),
        uiEvent,
        sender: tx,
    };

    (kbd, ar)
}
