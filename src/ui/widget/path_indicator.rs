use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::flex::Flex;
use crate::ui::widget::label::Label;
use crate::ui::widget::quoted::Quoted;
use crate::ui::{Functional, Mrc, ToMrc};
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::QueueableCommand;
use delegate::delegate;

pub struct PathIndicator {
    path: String,
    label: Mrc<Label>,
    main: Quoted,
}

impl PathIndicator {
    pub fn new(str: &str) -> Self {
        let label = Label::new(str).mrc();
        PathIndicator {
            path: str.to_string(),
            main: Quoted::new(label.clone()),
            label,
        }
    }

    pub fn set_path(&mut self, str: &str) {
        self.label.borrow_mut().set_text(str.to_string());
        self.clear();
        self.draw();
    }
}

draw_to! {
    PathIndicator.main
}
