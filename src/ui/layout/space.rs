use crate::ui::base::draw::{Draw, Drawable};

use crate::ui::base::shape::{Point, Rect, Size};
use fff_macros::draw_to;
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

#[draw_to(drawable)]
impl Draw for Space {
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
