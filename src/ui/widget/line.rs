use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crossterm::style::{Colors, Print, SetColors};
use crossterm::QueueableCommand;
use delegate::delegate;
use std::borrow::Borrow;
use std::cmp;
use std::io::stdout;

pub struct Line {
    drawable: Drawable,
    vertical: bool,
    vchar: char,
    hchar: char,
}

impl Line {
    pub fn new(vertical: bool) -> Self {
        Line::new_with_char(vertical, '│', '─')
    }

    fn new_with_char(vertical: bool, vchar: char, hchar: char) -> Self {
        Line {
            drawable: Drawable::new(),
            vertical,
            vchar,
            hchar,
        }
    }
}

impl Draw for Line {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn clear(&mut self);
            fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let s = if self.vertical {
            min.new_width(cmp::max(min.width, 1))
        } else {
            min.new_height(cmp::max(min.height, 1))
        };
        self.drawable.set_size(&s);
        s
    }

    fn do_draw(&mut self) {
        if self.vertical {
            let mut io = stdout();
            let p = self.get_rect().top_left();

            for i in 0..self.get_rect().get_height() {
                io.queue((&p + (0, i)).move_to())
                    .unwrap()
                    .queue(Print(self.vchar))
                    .unwrap();
            }
        } else {
            let mut s = String::new();
            for i in 0..self.get_rect().get_width() {
                s.push(self.hchar);
            }

            stdout()
                .queue(self.get_rect().top_left().move_to())
                .unwrap()
                .queue(Print(s))
                .unwrap();
        }
    }
}

pub struct DoubleLine {
    line: Line,
}

impl DoubleLine {
    pub fn new(vertical: bool) -> Self {
        DoubleLine {
            line: Line::new_with_char(vertical, '║', '═'),
        }
    }
}

impl Draw for DoubleLine {
    delegate! {
        to self.line {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn clear(&mut self);
            fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;
            fn do_draw(&mut self);
            fn ensure(&mut self, min: &Size, max: &Size) -> Size;
        }
    }
}
