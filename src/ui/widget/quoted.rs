use crate::ui::base::draw::{Draw};

use crate::ui::base::shape::{Point, Rect, Size};

use crate::ui::layout::flex::Flex;

use crate::ui::widget::label::Label;
use crate::ui::{Mrc, ToMrc};


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
