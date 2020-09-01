use crate::config::enums::BindingType;
use crate::config::Config;
use crate::kbd::mode::{KeyEventHandler, Mode};
use crate::ui::event::UIEventSender;
use crossbeam_channel::Sender;
use crossterm::event::KeyEvent;
use std::rc::Rc;

pub(super) struct IgnoreItHandler();

impl KeyEventHandler for IgnoreItHandler {
    fn handle(&mut self, _: KeyEvent) {}
}

pub struct NormalMode(Mode<IgnoreItHandler>);

impl NormalMode {
    pub fn new(config: &Config, sender: Sender<String>, ui_event: UIEventSender) -> Self {
        let bs = config.bindings(&BindingType::Normal);
        NormalMode(Mode::new("Normal", bs, sender, ui_event, IgnoreItHandler()))
    }

    pub(super) fn mode_mut(&mut self) -> &mut Mode<IgnoreItHandler> {
        &mut self.0
    }
}
