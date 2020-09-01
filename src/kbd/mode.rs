
use crate::config::{Action, Bindings};
use crate::kbd::code::key_event_code;
use crate::ui::event::{UIEvent, UIEventSender};
use crossbeam_channel::{Sender};
use crossterm::event::KeyEvent;
use std::collections::HashMap;


pub trait KeyEventHandler {
    fn handle(&mut self, ev: KeyEvent);
}

pub struct Mode<T: Sized + KeyEventHandler> {
    pub name: String,
    subs: Option<HashMap<String, String>>,
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

    fn set_subs(&mut self, subs: HashMap<String, String>) {
        let c: Vec<_> = subs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.ui_event.send_async(UIEvent::ShowKeyNav(c)).unwrap();
        self.subs = Some(subs);
    }
}

impl<T: Sized + KeyEventHandler> KeyEventHandler for Mode<T> {
    fn handle(&mut self, ev: KeyEvent) {
        let code = key_event_code(&ev);
        if let Some(sub) = &self.subs {
            if sub.contains_key(&code) {
                self.sender.send(sub.get(&code).unwrap().to_string());
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
                Action::Prefixed(m) => self.set_subs(m.clone()),
            },
            None => self.data.handle(ev),
        };
    }
}
