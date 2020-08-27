use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};

use crossterm::style::ResetColor;
use crossterm::QueueableCommand;
use delegate::delegate;

use crate::ui::Mrc;
use std::cell::{Ref, RefMut};
use std::io::stdout;
use std::ops::Deref;

pub trait Draw {
    fn get_rect(&self) -> &Rect;
    fn move_to(&mut self, point: &Point);
    fn ensure(&mut self, min: &Size, max: &Size) -> Size;
    fn do_draw(&mut self);
    fn clear(&mut self);

    fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>> {
        None
    }

    fn draw(&mut self) {
        self.do_draw();
        stdout().queue(ResetColor).unwrap();
    }
}

pub struct Drawable {
    pub rect: Rect,
    pub children: Vec<Mrc<dyn Draw>>,
}

impl Drawable {
    delegate! {
        to self.rect {
            pub fn set_x(&mut self, x: i16);
            pub fn set_y(&mut self, y: i16);
            pub fn set_position(&mut self, po: &Point);
            pub fn set_width(&mut self, width: u16);
            pub fn set_height(&mut self, height: u16);
            pub fn set_size(&mut self, size: &Size);
        }
    }

    pub fn new() -> Self {
        Drawable {
            rect: Rect::new(),
            children: Vec::new(),
        }
    }

    pub fn new_with_child(child: Mrc<dyn Draw>) -> Self {
        Drawable {
            rect: Rect::new(),
            children: vec![child],
        }
    }

    pub fn get_rect(&self) -> &Rect {
        &self.rect
    }

    pub fn clear(&mut self) {
        self.rect.clear()
    }

    pub fn move_to(&mut self, point: &Point) {
        self.rect.set_position(point);
    }

    pub fn mut_child(&mut self) -> RefMut<dyn Draw> {
        self.children[0].deref().borrow_mut()
    }

    pub fn child(&self) -> Ref<dyn Draw> {
        self.children[0].deref().borrow()
    }

    pub fn for_each<F: FnMut(RefMut<dyn Draw>)>(&self, mut f: F) {
        self.children
            .iter()
            .for_each(|it| f(it.deref().borrow_mut()));
    }

    pub fn for_each_indexed<F: FnMut(usize, RefMut<dyn Draw>)>(&self, mut f: F) {
        self.children
            .iter()
            .enumerate()
            .for_each(|(idx, it)| f(idx, it.deref().borrow_mut()));
    }

    pub fn fold<T, F: FnMut(T, RefMut<dyn Draw>) -> T>(&self, init: T, mut f: F) -> T {
        return self
            .children
            .iter()
            .fold(init, |acc, it| f(acc, it.deref().borrow_mut()));
    }
}
