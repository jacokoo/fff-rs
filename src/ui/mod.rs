use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::container::Container;
use crate::ui::layout::row::Row;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crossterm::terminal::size;
use std::cell::RefCell;
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

pub fn mrc<T>(t: T) -> Mrc<T> {
    Rc::new(RefCell::new(t))
}

pub fn demo() {
    let (width, height) = size().unwrap();
    let label = Label::new("hello".to_string()).mrc();
    let label2 = Label::new("world".to_string()).mrc();
    let label3 = Label::new("middle".to_string()).mrc();

    let mut row = Row::new();
    row.add(label.clone());
    row.add_flex(Space::new_with_width(10).mrc(), 1);
    row.add(label3.clone());
    row.add_flex(Space::new().mrc(), 2);
    row.add(label2.clone());

    let mr = row.mrc();
    let mut c = Container::new(mr.clone());
    let size = Size::new(width - 1, height - 1);
    c.ensure(&size, &size);
    c.move_to(&Point::new(1, 1));
    c.draw();

    println!("{}, {}!", width, height);
}
