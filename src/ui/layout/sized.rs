use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use std::borrow::BorrowMut;
use std::cell::{Ref, RefMut};
use std::cmp;
use std::ops::Deref;

pub struct SizedBox {
    inner: Size,
    drawable: Drawable,
    have_width: bool,
    have_height: bool,
}

impl SizedBox {
    pub fn new(child: Mrc<dyn Draw>) -> Self {
        SizedBox {
            inner: Size::new(0, 0),
            drawable: Drawable::new_with_child(child),
            have_width: false,
            have_height: false,
        }
    }

    pub fn width(mut self, v: u16) -> Self {
        self.inner.width = v;
        self.have_width = true;
        self
    }

    pub fn height(mut self, v: u16) -> Self {
        self.inner.height = v;
        self.have_height = true;
        self
    }

    pub fn size(self, w: u16, h: u16) -> Self {
        self.width(w).height(h)
    }

    pub fn max_width(mut self) -> Self {
        self.width(u16::max_value())
    }

    pub fn max_height(mut self) -> Self {
        self.height(u16::max_value())
    }

    pub fn max(self) -> Self {
        self.max_height().max_width()
    }
}

draw_to! {
    SizedBox.drawable

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let h = cmp::min(max.height, self.inner.height);
        let w = cmp::min(max.width, self.inner.width);

        let s = if !self.have_width && !self.have_height {
            self.drawable.mut_child().ensure(min, max)
        } else if !self.have_width {
            self.drawable
                .mut_child()
                .ensure(&min.new_height(h), &max.new_height(h))
        } else if !self.have_height {
            self.drawable
                .mut_child()
                .ensure(&min.new_width(w), &max.new_width(w))
        } else {
            let mm = Size::new(w, h);
            self.drawable.mut_child().ensure(&mm, &mm)
        };
        self.drawable.set_size(&s);
        s
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.drawable.mut_child().move_to(point);
    }

    fn do_draw(&mut self) {
        self.drawable.mut_child().draw();
    }
}
