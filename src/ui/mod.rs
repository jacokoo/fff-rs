use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::event::UIEventSender;
use crate::ui::main::event_handle::handle;
use crossterm::style::{Color, Colors};
use crossterm::terminal::size;
use main::ui::UI;
use std::cell::{RefCell, RefMut};
use std::io::{stdout, Write};
use std::ops::Deref;
use std::rc::Rc;
use std::thread;

mod base;
pub mod event;
mod layout;
mod main;
mod widget;

pub type Mrc<T> = Rc<RefCell<T>>;

pub trait ToMrc: Sized {
    fn mrc(self) -> Mrc<Self> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Sized + Draw> ToMrc for T {}

pub trait InnerFunctional<T: Sized + Draw> {
    fn inner_apply<F: FnOnce(RefMut<T>)>(&self, f: F);
    fn inner_also<F: FnOnce(RefMut<T>)>(self, f: F) -> Self;
}

impl<T: Sized + Draw> InnerFunctional<T> for Rc<RefCell<T>> {
    fn inner_apply<F: FnOnce(RefMut<T>)>(&self, f: F) {
        f(self.deref().borrow_mut());
    }

    fn inner_also<F: FnOnce(RefMut<T>)>(self, f: F) -> Self {
        f(self.deref().borrow_mut());
        self
    }
}

pub trait ColorNone {
    fn none() -> Self;
}

impl ColorNone for Colors {
    fn none() -> Self {
        Colors::new(Color::Reset, Color::Reset)
    }
}

pub fn init_ui(tab_count: usize) -> UIEventSender {
    let (sender, rx) = UIEventSender::new();
    thread::spawn(move || {
        let (width, height) = size().unwrap();
        let size = Size::new(width, height);

        let ui = UI::new(tab_count).also(|it| {
            it.ensure(&size, &size);
            it.move_to(&Point::new(0, 0));
            it.draw();
        });
        stdout().flush().unwrap();

        handle(ui, rx);
    });
    sender
}
