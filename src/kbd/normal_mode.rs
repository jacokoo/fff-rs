use crate::config::enums::BindingType;
use crate::config::Config;
use crate::kbd::mode::{KeyCodeAware, KeyEventHandler, Mode};
use crate::ui::event::UIEventSender;
use crossbeam_channel::Sender;
use crossterm::event::KeyEvent;

pub struct IgnoreItHandler();

impl KeyCodeAware for IgnoreItHandler {
    fn got_key(&mut self, _: &KeyEvent, _: Option<&str>) {}
}

pub struct NormalMode(Mode<IgnoreItHandler>);

impl NormalMode {
    pub fn new(config: &Config, sender: Sender<String>, ui_event: UIEventSender) -> Self {
        let bs = config.bindings(&BindingType::Normal);
        NormalMode(Mode::new(bs, sender, ui_event, IgnoreItHandler()))
    }

    pub fn name(&self) -> String {
        "Normal".to_string()
    }

    pub fn handle(&mut self, ev: KeyEvent) {
        self.0.handle(ev)
    }

    pub fn is_quit(&self, ev: &KeyEvent) -> bool {
        self.0.is_quit(ev)
    }
}
