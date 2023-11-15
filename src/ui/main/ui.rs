use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::layout::container::Container;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
use crate::ui::layout::space::Space;
use crate::ui::main::board::Board;
use crate::ui::main::input::Input;
use crate::ui::main::path_indicator::PathIndicator;
use crate::ui::main::statusbar::Statusbar;
use crate::ui::widget::label::Label;
use crate::ui::widget::tab::Tab;
use crate::ui::{InnerFunctional, Mrc, ToMrc};
use std::cell::RefMut;
use std::io::{stdout, Write};

pub struct UI {
    tab: Mrc<Tab>,
    path: Mrc<PathIndicator>,
    board: Mrc<Board>,
    statusbar: Mrc<Statusbar>,
    message: Mrc<Flex>,
    main: Container,
    loading: bool,
    show_message: u8,
    input: Mrc<Input>,
}

impl UI {
    pub fn new(tab_count: usize) -> Self {
        let ts: Vec<_> = (1..=tab_count).map(|it| it.to_string()).collect();
        let tab = Tab::new(ts, 0).mrc();
        let path = PathIndicator::new("").mrc();
        let board = Board::new().mrc();
        let statusbar = Statusbar::new().mrc();
        let message = Flex::row().mrc();
        let bottom = SizedBox::new(message.clone()).max_width().height(1).mrc();

        let top = Flex::row()
            .also(|it| {
                it.add(tab.clone());
                it.add(path.clone());
                it.add_flex(Space::new().mrc(), 1);
                it.add(Label::new("[?]").mrc());
            })
            .mrc();

        let main = Container::new(
            Flex::column()
                .also(|it| {
                    it.add(Padding::new(top.clone()).top_bottom(1).mrc());
                    it.add_flex(SizedBox::new(board.clone()).max().mrc(), 1);
                    it.add(statusbar.clone());
                    it.add(bottom.clone());
                })
                .mrc(),
        );

        UI {
            tab,
            path,
            board,
            statusbar,
            message,
            main,
            loading: false,
            show_message: 0,
            input: Input::new().mrc(),
        }
    }

    pub fn switch_tab(&mut self, current: usize) {
        self.tab.borrow_mut().set_active(current);
    }

    pub fn start_loading(&mut self) {
        if self.loading {
            return;
        }

        self.loading = true;
        self.statusbar.borrow_mut().set_spin(true);
    }

    pub fn stop_loading(&mut self) {
        if !self.loading {
            return;
        }

        self.loading = false;
        self.statusbar.borrow_mut().set_spin(false);
    }

    pub fn board_mut(&mut self) -> RefMut<Board> {
        self.board.borrow_mut()
    }

    pub fn path_mut(&mut self) -> RefMut<PathIndicator> {
        self.path.borrow_mut()
    }

    pub fn show_key_nav(&mut self, navs: Vec<(String, String)>) {
        self.show_message = 2;
        self.message.inner_apply(|mut it| {
            it.empty_it();
            navs.into_iter().for_each(|(key, msg)| {
                it.add_flex(Label::from(format!("[{}] {}", key, msg)).mrc(), 1);
            });
            it.redraw();
        });
    }

    pub fn show_input(&mut self, prompt: String) {
        self.show_message = 2;
        self.input.inner_apply(|mut it| it.init(prompt));
        self.message.inner_apply(|mut it| {
            it.empty_it();
            it.add(self.input.clone());
            it.redraw();
        });
        self.flush();
    }

    pub fn update_input(&mut self, input: String, cursor: usize) {
        self.input.borrow_mut().update(input, cursor);
    }

    pub fn update_input_cursor(&mut self, cursor: usize) {
        self.input.borrow_mut().move_cursor(cursor);
    }

    pub fn clear_message(&mut self) {
        if self.show_message == 0 {
            return;
        }

        self.show_message -= 1;
        if self.show_message != 0 {
            return;
        }

        self.show_message = 0;
        self.message.inner_apply(|mut it| {
            it.empty_it();
            it.clear();
        });
    }

    pub fn flush(&mut self) {
        stdout().flush().unwrap();
    }
}

#[draw_to(main)]
impl Draw for UI {}
