use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::container::Container;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crossterm::style::Print;
use crossterm::terminal::size;
use crossterm::{event, execute, queue};

use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::line::DoubleLine;
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
    let label = Label::new("hello".to_string()).mrc();
    let label2 = Label::new("world".to_string()).mrc();
    let label3 = Label::new("middle".to_string()).mrc();
    let line = DoubleLine::new(true).mrc();

    let pad = Padding::new(label2).left(2).mrc();

    let mut row = Flex::new(false);
    row.add(Padding::new(label.clone()).top_bottom(2).mrc());
    row.add(SizedBox::new(line.clone()).max_height().mrc());
    row.add(
        Padding::new(Label::new("padding top".to_string()).mrc())
            .top(2)
            .mrc(),
    );
    row.add_flex(Space::new_with_width(10).mrc(), 3);
    row.add(SizedBox::new(label3.clone()).width(20).mrc());
    row.add_flex(Space::new().mrc(), 2);
    row.add(pad.clone());

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
