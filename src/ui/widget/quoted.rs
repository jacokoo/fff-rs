use crate::ui::base::draw::Draw;

use crate::ui::base::shape::{Point, Rect, Size};

use crate::ui::layout::flex::Flex;

use crate::ui::widget::label::Label;
use crate::ui::{ColorNone, Functional, Mrc, ToMrc};
use crossterm::style::Colors;

pub struct Quoted {
    main: Flex,
    color: Colors,
}

impl Quoted {
    pub fn new<T: Draw + 'static>(child: Mrc<T>) -> Self {
        Quoted {
            main: Flex::row().also_mut(|it| {
                it.add(Label::new("[").mrc());
                it.add(child.clone());
                it.add(Label::new("]").mrc());
            }),
            color: Colors::none(),
        }
    }
}

#[draw_to(main)]
impl Draw for Quoted {}
