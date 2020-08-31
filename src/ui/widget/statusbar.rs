use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::background::Background;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::label::Label;
use crate::ui::widget::spinner::Spinner;
use crate::ui::{Mrc, ToMrc};
use crossterm::style::{Color, Colors};

pub struct Statusbar {
    spinner: Mrc<Spinner>,
    main: Background,
}

impl Statusbar {
    pub fn new() -> Self {
        let c = Colors::new(Color::Black, Color::Cyan);
        let sp = Spinner::new()
            .also_mut(|it| {
                it.set_color(c.clone());
            })
            .mrc();
        Statusbar {
            main: Background::new(
                SizedBox::new(
                    Flex::row()
                        .also_mut(|it| {
                            it.add(sp.clone());
                            it.add(
                                Label::new("status bar")
                                    .also_mut(|l| l.set_color(c.clone()))
                                    .mrc(),
                            )
                        })
                        .mrc(),
                )
                .max_width()
                .mrc(),
                Color::Cyan,
            ),
            spinner: sp,
        }
    }

    pub fn set_spin(&mut self, s: bool) {
        if s {
            self.spinner.borrow_mut().start();
        } else {
            self.spinner.borrow_mut().end();
        }
    }
}

#[draw_to(main)]
impl Draw for Statusbar {}
