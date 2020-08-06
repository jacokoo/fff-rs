use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Colors, Print, ResetColor, SetColors};
use crossterm::QueueableCommand;
use std::borrow::{Borrow, BorrowMut};
use std::io::{stdout, Write};
use std::iter::FromIterator;
use std::ops::Add;

pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Rect(Point, Size);

impl Add<(i16, i16)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i16, i16)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    pub fn move_to(&self) -> MoveTo {
        MoveTo(self.x as u16, self.y as u16)
    }
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size { width, height }
    }
}

impl Rect {
    pub fn new() -> Self {
        Rect(
            Point::new(0, 0),
            Size {
                width: 0,
                height: 0,
            },
        )
    }

    pub fn set_x(&mut self, x: i16) {
        self.0.x = x;
    }

    pub fn get_x(&self) -> i16 {
        self.0.x
    }

    pub fn set_y(&mut self, y: i16) {
        self.0.y = y;
    }

    pub fn get_y(&self) -> i16 {
        self.0.y
    }

    pub fn set_position(&mut self, po: &Point) {
        self.0.x = po.x;
        self.0.y = po.y;
    }

    pub fn set_width(&mut self, width: u16) {
        self.1.width = width
    }

    pub fn get_width(&self) -> u16 {
        self.1.width
    }

    pub fn set_height(&mut self, height: u16) {
        self.1.height = height;
    }

    pub fn get_height(&self) -> u16 {
        self.1.height
    }

    pub fn set_size(&mut self, size: &Size) {
        self.1.width = size.width;
        self.1.height = size.height;
    }

    pub fn top_left(&self) -> Point {
        Point::new(self.get_x(), self.get_y())
    }

    pub fn top_right(&self) -> Point {
        Point::new(self.get_x() + (self.get_width() as i16) - 1, self.get_y())
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(self.get_x(), self.get_y() + (self.get_height() as i16) - 1)
    }

    pub fn bottom_right(&self) -> Point {
        Point::new(
            self.get_x() + (self.get_width() as i16) - 1,
            self.get_y() + (self.get_height() as i16) - 1,
        )
    }

    pub fn clear(&self) {
        let tl = self.top_left();
        let br = self.bottom_right();
        let mut out = stdout();

        out.queue(ResetColor).unwrap();
        (tl.y..=br.y).enumerate().for_each(|(i, _)| {
            let cc = (tl.x..=br.x).map(|_| ' ').collect::<Vec<char>>();
            out.queue((&tl + (0i16, i as i16)).move_to()).unwrap();
            out.queue(Print(String::from_iter(cc))).unwrap();
        })
    }
}
