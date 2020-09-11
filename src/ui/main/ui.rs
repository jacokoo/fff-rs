use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::Size;
use crate::ui::layout::container::Container;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
use crate::ui::layout::space::Space;
use crate::ui::widget::board::Board;
use crate::ui::widget::label::Label;
use crate::ui::widget::path_indicator::PathIndicator;
use crate::ui::widget::statusbar::Statusbar;
use crate::ui::widget::tab::Tab;
use crate::ui::{Mrc, ToMrc};
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
            .also_mut(|it| {
                it.add(tab.clone());
                it.add(path.clone());
                it.add_flex(Space::new().mrc(), 1);
                it.add(Label::new("[?]").mrc());
            })
            .mrc();

        let main = Container::new(
            Flex::column()
                .also_mut(|it| {
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
        self.show_message = 1;
        self.message.borrow_mut().empty_it();
        navs.into_iter().for_each(|(key, msg)| {
            self.message
                .borrow_mut()
                .add_flex(Label::from(format!("[{}] {}", key, msg)).mrc(), 1);
        });
        self.message.borrow_mut().redraw();
    }

    pub fn flush(&mut self) {
        if self.show_message == 1 {
            self.show_message = 0
        } else {
            self.message.borrow_mut().empty_it();
            self.message.borrow_mut().redraw();
        }
        stdout().flush().unwrap();
    }
}

#[draw_to(main)]
impl Draw for UI {}
