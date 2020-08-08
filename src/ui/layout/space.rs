use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use delegate::delegate;
use std::cmp;

pub struct Space {
    width: u16,
    height: u16,
    drawable: Drawable,
}

impl Space {
    pub fn new() -> Self {
        Space {
            width: 0,
            height: 0,
            drawable: Drawable::new(),
        }
    }

    pub fn new_with_width(width: u16) -> Self {
        Space {
            width,
            height: 0,
            drawable: Drawable::new(),
        }
    }

    pub fn new_with_height(height: u16) -> Self {
        Space {
            width: 0,
            height,
            drawable: Drawable::new(),
        }
    }

    pub fn set(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}

impl Draw for Space {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn clear(&mut self);
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let s = Size::new(
            cmp::min(max.width, cmp::max(min.width, self.width)),
            cmp::min(max.height, cmp::max(min.height, self.height)),
        );

        self.drawable.set_size(&s);
        s
    }

    fn do_draw(&mut self) {}
}
