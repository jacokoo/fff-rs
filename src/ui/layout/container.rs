use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::widget::label::Label;
use crate::ui::Mrc;
use delegate::delegate;
use std::any::Any;
use std::borrow::BorrowMut;
use std::ops::Deref;

pub struct Container {
    drawable: Drawable,
    child: Mrc<dyn Draw>,
}

impl Container {
    pub fn new<T: Draw + 'static>(child: Mrc<T>) -> Self {
        Container {
            drawable: Drawable::new(),
            child,
        }
    }
}

impl Draw for Container {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn clear(&mut self);
            fn is_drawn(&self) -> bool;
            fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;
        }
    }

    fn ensure(&mut self, _: &Size, max: &Size) -> Size {
        let s = Size::new(max.width, max.height);
        self.drawable.set_size(&s);
        self.child.deref().borrow_mut().ensure(&s, &s);
        s
    }

    fn do_draw(&mut self) {
        self.child.deref().borrow_mut().draw();
        self.drawable.drawn = true;
    }
}
