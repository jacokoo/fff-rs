use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use crossterm::style::ResetColor;
use crossterm::QueueableCommand;
use delegate::delegate;
use std::any::Any;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

pub trait Draw: Any {
    fn get_rect(&self) -> &Rect;
    fn move_to(&mut self, point: &Point);
    fn ensure(&mut self, min: &Size, max: &Size) -> Size;
    fn do_draw(&mut self);
    fn clear(&mut self);

    fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;

    fn draw(&mut self) {
        self.clear();
        self.do_draw();
        stdout().queue(ResetColor).unwrap();
    }
}

pub struct Drawable {
    pub rect: Rect,
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
        Drawable { rect: Rect::new() }
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

    pub fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>> {
        None
    }
}
