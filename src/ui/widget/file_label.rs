use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::background::Background;
use crate::ui::widget::label::Label;
use crate::ui::{Functional, Mrc, ToMrc};
use crossterm::style::{Color, Colors};
use delegate::delegate;
use std::borrow::BorrowMut;
use std::ops::Deref;

trait InverseColor {
    fn inverse(&self) -> Self;
}

impl InverseColor for Colors {
    fn inverse(&self) -> Self {
        Colors::new(
            self.background.unwrap_or(Color::Reset),
            self.foreground.unwrap_or(Color::Reset),
        )
    }
}

pub struct FileLabel {
    label: Mrc<Label>,
    selected: bool,
    marked: bool,
    color: Colors,
    marked_color: Colors,
    marker: Label,
    background: Background,
}

impl FileLabel {
    pub fn from(txt: String) -> Self {
        Colors::new(Color::Reset, Color::Reset).map_to(|c| {
            let label = Label::from(format!("  {}  ", txt))
                .also_mut(|it| it.set_color(c.clone()))
                .mrc();
            return FileLabel {
                selected: false,
                marked: false,
                color: c,
                marked_color: c.clone(),
                marker: Label::new("*"),
                background: Background::new(label.clone(), Color::Reset),
                label,
            };
        })
    }

    pub fn new(txt: &str) -> Self {
        Self::from(txt.to_string())
    }

    pub fn set_color(&mut self, c: Colors) {
        self.color = c;
        self.ensure_color();
    }

    pub fn set_selected(&mut self, s: bool) {
        self.selected = s;
        self.ensure_color();
    }

    pub fn set_marked(&mut self, s: bool) {
        self.marked = s;
        self.ensure_color();
    }

    pub fn set_marked_color(&mut self, c: Colors) {
        self.marked_color = c;
        self.ensure_color();
    }

    fn ensure_color(&mut self) {
        let mut used_color = if self.marked {
            self.marked_color
        } else {
            self.color
        };

        if self.selected {
            used_color = used_color.inverse();
        }

        self.label.deref().borrow_mut().set_color(used_color);
        self.marker.set_color(used_color.clone());
        self.background.set_color(used_color);
    }
}

impl Draw for FileLabel {
    delegate! {
        to self.background {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn clear(&mut self);
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.marker.ensure(min, max);
        self.background.ensure(min, max)
    }

    fn do_draw(&mut self) {
        self.background.draw();
        if self.marked {
            self.marker
                .move_to(&(&self.background.get_rect().top_left() + (1u16, 0u16)));
            self.marker.draw()
        }
    }
}
