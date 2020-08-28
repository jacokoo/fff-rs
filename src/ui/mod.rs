mod ui;

use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::container::Container;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crossterm::style::{Color, Colors};
use crossterm::terminal::size;

use crate::ui::layout::background::Background;
use crate::ui::layout::center::Center;
use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::board::Board;
use crate::ui::widget::file_label::FileLabel;
use crate::ui::widget::file_list::FileList;
use crate::ui::widget::line::Line;
use crate::ui::widget::path_indicator::PathIndicator;
use crate::ui::widget::tab::Tab;
use std::cell::RefCell;

use crate::ui::widget::spinner::Spinner;
use std::rc::Rc;

mod base;
mod event;
mod layout;
mod widget;

pub type Mrc<T> = Rc<RefCell<T>>;

pub trait ToMrc: Sized {
    fn mrc(self) -> Mrc<Self> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Sized + Draw> ToMrc for T {}

pub trait Functional: Sized {
    fn also<F: FnOnce(&Self)>(self, f: F) -> Self {
        f(&self);
        self
    }

    fn also_mut<F: FnMut(&mut Self)>(mut self, mut f: F) -> Self {
        f(&mut self);
        self
    }

    fn map_to<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
        return f(self);
    }
}

impl<T: Sized> Functional for T {}

pub trait ColorNone {
    fn none() -> Self;
}

impl ColorNone for Colors {
    fn none() -> Self {
        Colors::new(Color::Reset, Color::Reset)
    }
}

fn create_file(txt: &str, dir: bool) -> Mrc<FileLabel> {
    FileLabel::new(txt)
        .also_mut(|it| {
            it.set_marked_color(Colors::new(Color::Yellow, Color::Black));
            if dir {
                it.set_color(Colors::new(Color::Cyan, Color::Black));
            } else {
                it.set_color(Colors::new(Color::White, Color::Black));
            }
        })
        .mrc()
}

pub fn demo() {
    let (width, height) = size().unwrap();
    let tab = Tab::new(
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ],
        0,
    )
    .mrc();

    let path = PathIndicator::new("/Users/guyong/ws/rust/fff").mrc();

    let top = Flex::row()
        .also_mut(|it| {
            it.add(tab.clone());
            it.add(path.clone());
            it.add_flex(Space::new().mrc(), 1);
            it.add(Label::new("[?]").mrc());
        })
        .mrc();

    let status_bar = Background::new(
        Flex::row()
            .also_mut(|it| {
                it.add(
                    Label::new("status bar")
                        .also_mut(|it| it.set_color(Colors::new(Color::Black, Color::Cyan)))
                        .mrc(),
                );
                it.add_flex(Space::new().mrc(), 1);
                it.add(Spinner::new().mrc());
            })
            .mrc(),
        Color::Cyan,
    )
    .mrc();

    let board = Board::new()
        .also_mut(|it| {
            it.add_file_list(
                FileList::new()
                    .also_mut(|it| {
                        it.set_files(vec![
                            create_file("hello", true),
                            create_file("hello", false),
                            create_file("hello world", false),
                        ]);
                        it.set_selected(Some(1));
                        it.set_marked(vec![0, 1]);
                    })
                    .mrc(),
            );

            it.add_bookmark("Home".to_string());
            it.add_bookmark("Root".to_string());
        })
        .mrc();

    let mut root = Container::new(
        Flex::column()
            .also_mut(|it| {
                it.add(Padding::new(top.clone()).top_bottom(1).mrc());
                it.add_flex(SizedBox::new(board.clone()).max().mrc(), 1);
                it.add(SizedBox::new(status_bar.clone()).max_width().mrc());
                it.add(SizedBox::new(Space::new().mrc()).height(1).mrc());
            })
            .mrc(),
    );

    // let mut root = Container::new(tab.clone());

    let size = Size::new(width, height);
    root.ensure(&size, &size);
    root.move_to(&Point::new(0, 0));
    root.draw();

    // println!("{:?}", tab.borrow().get_rect());
}
