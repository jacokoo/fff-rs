use crate::config::{Action, Bindings};
use crate::kbd::code::key_event_code;
use crate::ui::event::{UIEvent, UIEventSender};
use crossbeam_channel::Sender;
use crossterm::event::KeyEvent;
use std::collections::HashMap;

const QUIT_ACTION: &'static str = "ActionQuit";

pub trait KeyEventHandler {
    fn handle(&mut self, ev: KeyEvent);
}

pub struct Mode<T: Sized + KeyEventHandler> {
    pub name: String,
    subs: Option<HashMap<String, (String, String)>>,
    bindings: Bindings,
    sender: Sender<String>,
    ui_event: UIEventSender,
    data: T,
}

impl<T: Sized + KeyEventHandler> Mode<T> {
    pub(super) fn new(
        name: &str,
        bindings: Bindings,
        sender: Sender<String>,
        ui_event: UIEventSender,
        data: T,
    ) -> Self {
        Mode {
            name: name.to_string(),
            subs: None,
            bindings,
            sender,
            ui_event,
            data,
        }
    }

    pub(super) fn is_quit(&self, ev: &KeyEvent) -> bool {
        let code = key_event_code(ev);
        if let Some(v) = &self.subs {
            return match v.get(&code) {
                Some((v, _)) if v == QUIT_ACTION => true,
                _ => false,
            };
        }

        if let Some(v) = self.bindings.get(&code) {
            return match v {
                Action::Normal(v) if v == QUIT_ACTION => true,
                _ => false,
            };
        }

        return false;
    }
}

impl<T: Sized + KeyEventHandler> KeyEventHandler for Mode<T> {
    fn handle(&mut self, ev: KeyEvent) {
        let code = key_event_code(&ev);
        if let Some(sub) = &self.subs {
            if sub.contains_key(&code) {
                let x = sub.get(&code).unwrap();
                self.sender.send(x.0.to_string()).unwrap();
            } else {
                self.data.handle(ev);
            }
            self.subs = None;
            return;
        }

        let action = self.bindings.get(&code);
        match action {
            Some(a) => match a {
                Action::Normal(s) => self.sender.send(s.to_string()).unwrap(),
                Action::Prefixed(m) => {
                    let c: Vec<_> = m
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.1.to_string()))
                        .collect();
                    self.ui_event.send(UIEvent::ShowKeyNav(c)).unwrap();
                    self.subs = Some(m.clone());
                }
            },
            None => self.data.handle(ev),
        };
    }
}
