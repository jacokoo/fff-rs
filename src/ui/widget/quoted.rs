use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::container::UseMin;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::label::Label;
use crate::ui::{Functional, Mrc, ToMrc};
use delegate::delegate;

pub struct Quoted {
    main: Flex,
}

impl Quoted {
    pub fn new<T: Draw + 'static>(child: Mrc<T>) -> Self {
        let mut it = Flex::row();
        it.add(Label::new("[").mrc());
        it.add(child);
        it.add(Label::new("]").mrc());
        Quoted { main: it }
    }
}

draw_to! {
    Quoted.main
}
