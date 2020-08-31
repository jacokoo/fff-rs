use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::flex::Flex;
use crate::ui::widget::label::Label;
use crate::ui::{ColorNone, Mrc, ToMrc};
use crossterm::style::Colors;

pub struct Quoted {
    main: Flex,
    lr: Mrc<Label>,
    ll: Mrc<Label>,
}

impl Quoted {
    pub fn new<T: Draw + 'static>(child: Mrc<T>) -> Self {
        let ll = Label::new("[").mrc();
        let lr = Label::new("]").mrc();
        Quoted {
            main: Flex::row().also_mut(|it| {
                it.add(ll.clone());
                it.add(child.clone());
                it.add(lr.clone());
            }),
            lr,
            ll,
        }
    }

    pub fn set_color(&mut self, color: Colors) {
        self.lr.borrow_mut().set_color(color.clone());
        self.ll.borrow_mut().set_color(color);
    }
}

#[draw_to(main)]
impl Draw for Quoted {}
