use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use delegate::delegate;
use std::borrow::BorrowMut;
use std::cmp;
use std::ops::Deref;

pub struct SizedBox {
    inner: Size,
    drawable: Drawable,
    child: Mrc<dyn Draw>,
}

impl SizedBox {
    pub fn new(child: Mrc<dyn Draw>) -> Self {
        SizedBox {
            inner: Size::new(0, 0),
            drawable: Drawable::new(),
            child,
        }
    }

    pub fn width(mut self, v: u16) -> Self {
        self.inner.width = v;
        self
    }

    pub fn height(mut self, v: u16) -> Self {
        self.inner.height = v;
        self
    }

    pub fn max_width(mut self) -> Self {
        self.inner.width = u16::max_value();
        self
    }

    pub fn max_height(mut self) -> Self {
        self.inner.height = u16::max_value();
        self
    }

    pub fn max(self) -> Self {
        self.max_height().max_width()
    }

    pub fn size(self, w: u16, h: u16) -> Self {
        self.width(w).height(h)
    }
}

impl Draw for SizedBox {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn clear(&mut self);
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let mut s = Size::new(
            cmp::min(max.width, cmp::max(min.width, self.inner.width)),
            cmp::min(max.height, cmp::max(min.height, self.inner.height)),
        );
        let si = self.child.deref().borrow_mut().ensure(&s, &s);
        s.keep_max(&si);
        self.drawable.set_size(&s);
        s
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.child.deref().borrow_mut().move_to(point);
    }

    fn do_draw(&mut self) {
        self.child.deref().borrow_mut().draw();
    }
}
