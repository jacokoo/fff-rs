use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use delegate::delegate;
use std::borrow::BorrowMut;
use std::ops::Deref;

pub struct Container {
    drawable: Drawable,
}

impl Container {
    pub fn new<T: Draw + 'static>(child: Mrc<T>) -> Self {
        Container {
            drawable: Drawable::new_with_child(child),
        }
    }
}

impl Draw for Container {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn clear(&mut self);
        }
    }

    fn ensure(&mut self, _: &Size, max: &Size) -> Size {
        let s = Size::new(max.width, max.height);
        self.drawable.set_size(&s);
        self.drawable.mut_child().ensure(&Size::new(0, 0), &s);
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

pub struct UseMin {
    width: bool,
    height: bool,
    drawable: Drawable,
}

impl UseMin {
    pub fn new(child: Mrc<dyn Draw>, height: bool, width: bool) -> Self {
        UseMin {
            width,
            height,
            drawable: Drawable::new_with_child(child),
        }
    }

    pub fn height(child: Mrc<dyn Draw>) -> Self {
        Self::new(child, true, false)
    }

    pub fn width(child: Mrc<dyn Draw>) -> Self {
        Self::new(child, false, true)
    }
}

impl Draw for UseMin {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn clear(&mut self);
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let mi = min.clone();
        let mut ma = max.clone();
        if self.height {
            ma.height = min.height;
        }
        if self.width {
            ma.width = min.width;
        }
        self.drawable.mut_child().ensure(&mi, &ma)
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.drawable.mut_child().move_to(point);
    }

    fn do_draw(&mut self) {
        self.drawable.mut_child().draw();
    }
}
