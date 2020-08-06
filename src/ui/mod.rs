use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::container::Container;
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
    let mut label = Label::new("ℝℝℝℝℝ".to_string()).mrc();
    let mut c = Container::new(label.clone());
    let size = Size::new(width, height);
    c.ensure(&size, &size);
    c.draw();

    println!("{}", label.borrow().get_rect().get_height());
}
