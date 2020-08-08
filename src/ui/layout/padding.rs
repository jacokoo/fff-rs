use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use delegate::delegate;

use std::ops::Deref;

pub struct Padding {
    padding: (u16, u16, u16, u16),
    drawable: Drawable,
    child: Mrc<dyn Draw>,
}

impl Padding {
    pub fn new(child: Mrc<dyn Draw>) -> Self {
        Self {
            padding: (0, 0, 0, 0),
            drawable: Drawable::new(),
            child,
        }
    }

    pub fn top(mut self, v: u16) -> Self {
        self.padding.0 = v;
        self
    }

    pub fn bottom(mut self, v: u16) -> Self {
        self.padding.1 = v;
        self
    }

    pub fn left(mut self, v: u16) -> Self {
        self.padding.2 = v;
        self
    }

    pub fn right(mut self, v: u16) -> Self {
        self.padding.3 = v;
        self
    }

    pub fn top_bottom(mut self, v: u16) -> Self {
        self.padding.0 = v;
        self.padding.1 = v;
        self
    }

    pub fn left_right(mut self, v: u16) -> Self {
        self.padding.2 = v;
        self.padding.3 = v;
        self
    }
}

impl Draw for Padding {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn clear(&mut self);
            fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let (t, b, l, r) = self.padding;
        let s = (l + r, t + b);

        let si = self
            .child
            .deref()
            .borrow_mut()
            .ensure(&(min - s), &(max - s));

        let si2 = &si + s;
        self.drawable.set_size(&si2);
        si2
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.child
            .deref()
            .borrow_mut()
            .move_to(&(point + (self.padding.2, self.padding.0)))
    }

    fn do_draw(&mut self) {
        self.child.deref().borrow_mut().draw()
    }
}
