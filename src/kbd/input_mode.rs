use crate::config::enums::BindingType;
use crate::config::Config;
use crate::kbd::mode::{KeyCodeAware, KeyEventHandler, Mode};
use crate::kbd::Answer;
use crate::ui::event::UIEventSender;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::event::{KeyCode, KeyEvent};

pub const INPUT_QUIT_ACTION: &'static str = "ActionQuitInputMode";
pub const INPUT_ABORT_ACTION: &'static str = "ActionAbortInputMode";
pub const INPUT_DELETE_ACTION: &'static str = "ActionInputDelete";
pub const INPUT_DELETE_BACKWARD_ACTION: &'static str = "ActionInputDeleteBackward";
pub const INPUT_MOVE_BACK: &'static str = "ActionInputMoveBack";
pub const INPUT_MOVE_FORWARD: &'static str = "ActionInputMoveForward";
pub const INPUT_MOVE_TO_START: &'static str = "ActionInputMoveToStart";
pub const INPUT_MOVE_TO_END: &'static str = "ActionInputMoveToEnd";

pub struct AnswerInput {
    result: Sender<Option<Answer>>,
    sender: Sender<String>,
    ui_event: UIEventSender,
    multiple: bool,
}

impl AnswerInput {
    fn send_result(&mut self, re: Option<Answer>) {
        self.result.send(re).unwrap();
        self.sender.send(INPUT_QUIT_ACTION.to_string()).unwrap();
    }
}

impl KeyCodeAware for AnswerInput {
    fn got_key(&mut self, code: &KeyEvent, action: Option<&str>) {
        match action {
            Some(INPUT_ABORT_ACTION) | Some(INPUT_QUIT_ACTION) => self.result.send(None).unwrap(),
            _ => match code.code {
                KeyCode::Char('y') => {
                    self.send_result(Some(Answer::Yes));
                }
                KeyCode::Char('n') => {
                    self.send_result(Some(Answer::No));
                }
                KeyCode::Char('Y') if self.multiple => {
                    self.send_result(Some(Answer::YesToAll));
                }
                KeyCode::Char('N') if self.multiple => {
                    self.send_result(Some(Answer::NoToAll));
                }
                _ => {}
            },
        }
    }
}

pub struct NormalInput {
    result: Sender<Option<String>>,
    sender: Sender<String>,
    ui_event: UIEventSender,
    input: String,
    cursor: usize,
}

impl NormalInput {
    fn delete(&mut self, backward: bool) {}
    fn move_delta(&mut self, delta: i16) {}
    fn move_to(&mut self, end: bool) {}
    fn append(&mut self, c: char) {}
}

impl KeyCodeAware for NormalInput {
    fn got_key(&mut self, code: &KeyEvent, action: Option<&str>) {
        match action {
            Some(INPUT_QUIT_ACTION) => self.result.send(Some(self.input.clone())).unwrap(),
            Some(INPUT_ABORT_ACTION) => self.result.send(None).unwrap(),
            Some(INPUT_DELETE_ACTION) => self.delete(false),
            Some(INPUT_DELETE_BACKWARD_ACTION) => self.delete(true),
            Some(INPUT_MOVE_BACK) => self.move_delta(-1),
            Some(INPUT_MOVE_FORWARD) => self.move_delta(1),
            Some(INPUT_MOVE_TO_START) => self.move_to(false),
            Some(INPUT_MOVE_TO_END) => self.move_to(true),
            _ => {
                if let KeyCode::Char(c) = code.code {
                    self.append(c);
                }
            }
        }
    }
}

pub enum InputMode {
    Answer(Mode<AnswerInput>),
    Input(Mode<NormalInput>),
}

impl InputMode {
    pub fn handle(&mut self, ev: KeyEvent) {
        match self {
            Self::Answer(m) => m.handle(ev),
            Self::Input(m) => m.handle(ev),
        }
    }

    pub fn name(&self) -> String {
        "Input".to_string()
    }

    pub fn new_answer(
        config: &Config,
        sender: Sender<String>,
        ui_event: UIEventSender,
        multiple: bool,
    ) -> (Self, Receiver<Option<Answer>>) {
        let (tx, rx) = bounded(0);
        let ai = AnswerInput {
            result: tx,
            sender: sender.clone(),
            ui_event: ui_event.clone(),
            multiple,
        };

        (
            InputMode::Answer(Mode::new(
                config.bindings(&BindingType::Input),
                sender,
                ui_event,
                ai,
            )),
            rx,
        )
    }

    pub fn new_input(
        config: &Config,
        sender: Sender<String>,
        ui_event: UIEventSender,
    ) -> (Self, Receiver<Option<String>>) {
        let (tx, rx) = bounded(0);

        let ni = NormalInput {
            result: tx,
            sender: sender.clone(),
            ui_event: ui_event.clone(),
            input: "".to_string(),
            cursor: 0,
        };

        (
            InputMode::Input(Mode::new(
                config.bindings(&BindingType::Input),
                sender,
                ui_event,
                ni,
            )),
            rx,
        )
    }
}
