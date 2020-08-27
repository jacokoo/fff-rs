use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::{Functional, Mrc};
use std::borrow::BorrowMut;
use std::ops::{Deref, Div};

pub struct Center {
    drawable: Drawable,
}

impl Center {
    pub fn new<T: Draw + 'static>(child: Mrc<T>) -> Self {
        Center {
            drawable: Drawable::new_with_child(child),
        }
    }
}

draw_to! {
    Center.drawable

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let mut s = self.drawable.mut_child().ensure(&Size::new(0, 0), max);
        s.keep_max(min);
        self.drawable.set_size(&s);
        s
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        let mut cw = self.drawable.child().get_rect().get_width();
        let mut ch = self.drawable.child().get_rect().get_height();
        cw = (self.get_rect().get_width() - cw) / 2;
        ch = (self.get_rect().get_height() - ch) / 2;

        self.drawable.mut_child().move_to(&(point + (cw, ch)));
    }

    fn do_draw(&mut self) {
        self.drawable.mut_child().draw();
    }
}
