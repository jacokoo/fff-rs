use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::flex::Flex;
use crate::ui::widget::label::Label;
use crate::ui::{Functional, Mrc, ToMrc};
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::QueueableCommand;
use delegate::delegate;

pub struct PathIndicator {
    path: String,
    label: Mrc<Label>,
    flex: Flex,
}

impl PathIndicator {
    pub fn new(str: &str) -> Self {
        let label = Label::new(str).mrc();
        PathIndicator {
            path: str.to_string(),
            flex: Flex::new(false).also_mut(|it| {
                it.add(Label::new("[").mrc());
                it.add(label.clone());
                it.add(Label::new("]").mrc());
            }),
            label,
        }
    }

    pub fn set_path(&mut self, str: &str) {
        self.label.borrow_mut().set_text(str.to_string());
        self.clear();
        self.draw();
    }
}

impl Draw for PathIndicator {
    delegate! {
        to self.flex {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn ensure(&mut self, min: &Size, max: &Size) -> Size;
            fn do_draw(&mut self);
            fn clear(&mut self);
        }
    }
}
