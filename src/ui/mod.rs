use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::container::Container;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crossterm::style::{Color, Colors, Print};
use crossterm::terminal::size;
use crossterm::{event, execute, queue};

use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
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
        .add(tab.clone())
        .add(path.clone())
        .add_flex(Space::new().mrc(), 1)
        .add(Label::new("[?]").mrc())
        .mrc();

    let mut root = Container::new(
        SizedBox::new(
            Flex::new(true)
                .add(Padding::new(top).top_bottom(1).mrc())
                .add(SizedBox::new(Line::new(false).mrc()).max_width().mrc())
                .mrc(),
        )
        .max_width()
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
        .add(Padding::new(label.clone()).top_bottom(2).mrc())
        .add(SizedBox::new(line.clone()).max_height().mrc())
        .add(Padding::new(Label::new("padding top").mrc()).top(2).mrc())
        .add_flex(Space::new_with_width(10).mrc(), 3)
        .add(SizedBox::new(label3.clone()).width(20).mrc())
        .add_flex(Space::new().mrc(), 2)
        .add(pad.clone());

    let mr = row.mrc();
    let mut c = Container::new(mr.clone());
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
