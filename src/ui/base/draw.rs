use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crossterm::style::ResetColor;
use crossterm::QueueableCommand;
use std::cell::RefCell;
use std::io::{stdout, Write};

pub trait Draw {
    fn id(&self) -> u16;
    fn get_rect(&self) -> &Rect;
    fn move_to(&mut self, point: &Point);
    fn ensure(&mut self, size: &Size) -> Size;
    fn is_drawn(&self) -> bool;
    fn do_draw(&mut self);

    fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;

    fn draw(&mut self) {
        if self.is_drawn() {
            self.clear();
        }
        self.do_draw();
        stdout().queue(ResetColor).unwrap();
    }

    fn clear(&self) {
        self.get_rect().clear()
    }
}

pub struct Counter(u16);

impl Counter {
    pub fn next(&mut self) -> u16 {
        let i = self.0;
        self.0 += 1;
        return i;
    }
}

pub const COUNTER: Counter = Counter(0);
