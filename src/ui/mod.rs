use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::container::Container;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crossterm::style::{Color, Colors, Print};
use crossterm::terminal::size;
use crossterm::{event, execute, queue};

use crate::ui::layout::background::Background;
use crate::ui::layout::center::Center;
use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::file_label::FileLabel;
use crate::ui::widget::file_list::FileList;
use crate::ui::widget::line::{DoubleLine, Line};
use crate::ui::widget::path_indicator::PathIndicator;
use crate::ui::widget::tab::Tab;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

mod base;
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
    fn also<F: FnOnce(&Self)>(self, mut f: F) -> Self {
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

    let top = Flex::new(false)
        .also_mut(|it| {
            it.add(tab.clone());
            it.add(path.clone());
            it.add_flex(Space::new().mrc(), 1);
            it.add(Label::new("[?]").mrc());
        })
        .mrc();

    let status_bar = Background::new(
        Flex::new(false)
            .also_mut(|it| {
                it.add(
                    Label::new("status bar")
                        .also_mut(|it| it.set_color(Colors::new(Color::Black, Color::Cyan)))
                        .mrc(),
                );
                it.add_flex(Space::new().mrc(), 1);
                it.add(Label::new("status bar end").mrc());
            })
            .mrc(),
        Color::Cyan,
    )
    .mrc();

    let bookmark = Flex::new(true)
        .also_mut(|it| {
            it.set_stretch();
            it.add(
                Padding::new(Center::new(Label::new("BOOKMARK").mrc()).mrc())
                    .top_bottom(1)
                    .left_right(2)
                    .mrc(),
            );
            it.add(SizedBox::new(Line::new(false).mrc()).mrc());
            it.add(Label::new("hello").mrc());
            it.add(Label::new("hello hello hello").mrc());
        })
        .mrc();

    let cols = Flex::new(false)
        .also_mut(|it| {
            it.set_stretch();
            it.add(bookmark.clone());
            it.add(DoubleLine::new(true).mrc());
            it.add(
                SizedBox::new(
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
                )
                .width(30)
                .max_height()
                .mrc(),
            );
            it.add(Line::new(true).mrc());
        })
        .mrc();

    let mut root = Container::new(
        Flex::new(true)
            .also_mut(|it| {
                it.add(Padding::new(top.clone()).top_bottom(1).mrc());
                it.add(SizedBox::new(Line::new(false).mrc()).max_width().mrc());
                it.add_flex(cols.clone(), 1);
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

pub fn demo1() {
    let (width, height) = size().unwrap();
    let label = Label::new("hello").mrc();
    let label2 = Label::new("world").mrc();
    let label3 = Label::new("middle").mrc();
    let line = DoubleLine::new(true).mrc();

    let pad = Padding::new(label2).left(2).mrc();

    let mut row = Flex::new(false)
        .also_mut(|it| {
            it.add(Padding::new(label.clone()).top_bottom(2).mrc());
            it.add(SizedBox::new(line.clone()).max_height().mrc());
            it.add(Padding::new(Label::new("padding top").mrc()).top(2).mrc());
            it.add_flex(Space::new_with_width(10).mrc(), 3);
            it.add(SizedBox::new(label3.clone()).width(20).mrc());
            it.add_flex(Space::new().mrc(), 2);
            it.add(pad.clone());
        })
        .mrc();

    let mut c = Container::new(row.clone());
    let size = Size::new(width - 2, height);
    c.ensure(&size, &size);
    c.move_to(&Point::new(2, 0));
    c.draw();

    for i in 0..56 {
        execute!(
            stdout(),
            Point::new(0, i).move_to(),
            Print(format!("{}", i))
        )
        .unwrap();
    }
    execute!(stdout(), Point::new(0, 54).move_to()).unwrap();
}
