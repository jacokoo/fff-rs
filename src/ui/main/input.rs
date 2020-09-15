use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::layout::flex::Flex;
use crate::ui::widget::label::Label;
use crate::ui::{InnerFunctional, Mrc, ToMrc};
use crossterm::cursor::{Hide, Show};
use crossterm::QueueableCommand;
use std::io::stdout;

pub struct Input {
    main: Flex,
    input: Mrc<Label>,
    cursor: usize,
}

impl Input {
    pub fn new() -> Self {
        Input {
            main: Flex::row(),
            input: Label::new("").mrc(),
            cursor: 0,
        }
    }

    pub fn init(&mut self, prompt: String) {
        self.cursor = 0;
        let ii = self.input.clone().inner_also(|mut it| {
            it.set_text("".to_string());
        });
        self.main.apply(|it| {
            it.empty_it();
            it.add(Label::from(format!("{}: ", prompt)).mrc());
            it.add(ii);
        });
    }

    pub fn update(&mut self, text: String, cursor: usize) {
        self.cursor = cursor;
        self.input.inner_apply(|mut it| {
            it.set_text(text);
            it.redraw();

            let m = (&it.get_rect().top_left().delta_x(self.cursor as i16)).cursor();
            stdout().queue(m).unwrap();
        });
    }

    pub fn move_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
        stdout()
            .queue((&self.input.borrow().get_rect().top_left().delta_x(self.cursor as i16)).cursor())
            .unwrap();
    }
}

#[draw_to(main)]
impl Draw for Input {
    fn do_draw(&mut self) {
        self.main.do_draw();
        stdout()
            .queue(Show)
            .unwrap()
            .queue((&self.input.borrow().get_rect().top_left().delta_x(self.cursor as i16)).cursor())
            .unwrap();
    }

    fn clear(&mut self) {
        stdout().queue(Hide).unwrap();
        self.main.clear();
    }
}
