use crate::ui::base::draw::{Draw, Drawable};

use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::ColorNone;
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::QueueableCommand;

use std::cmp;
use std::io::stdout;

pub struct Label {
    drawable: Drawable,
    text: String,
    text_width: u16,
    colors: Colors,
}

impl Label {
    pub fn from(txt: String) -> Self {
        Label {
            drawable: Drawable::new(),
            text_width: Label::width(&txt),
            text: txt,
            colors: Colors::none(),
        }
    }

    pub fn new(txt: &str) -> Self {
        Label::from(txt.to_string())
    }

    pub fn set_text(&mut self, txt: String) {
        self.text_width = Label::width(&txt);
        self.text = txt;
    }

    pub fn set_color(&mut self, colors: Colors) {
        self.colors = colors;
    }

    pub fn reset_color(&mut self) {
        self.colors = Colors::none();
    }

    fn char_width(c: char) -> u16 {
        unicode_width::UnicodeWidthChar::width(c).unwrap_or(if c.len_utf8() > 2 { 2 } else { 1 })
            as u16
    }

    fn width(text: &String) -> u16 {
        text.chars().fold(0, |a, c| a + Label::char_width(c))
    }

    fn truncate(text: &String, width: u16) -> String {
        let mut w = width;
        let mut re = "".to_string();
        text.chars().find_map(|c| {
            let cw = Label::char_width(c);
            if cw <= w {
                w -= cw;
                re.push(c);
                return None;
            }
            Some(())
        });
        re
    }

    fn max_width_to(text: &String, width: u16) -> u16 {
        let mut w = width;
        let mut re = 0u16;
        text.chars().find_map(|c| {
            let cw = Label::char_width(c);
            if cw <= w {
                w -= cw;
                re += cw;
                return None;
            }
            Some(())
        });
        re
    }
}

#[draw_to(drawable)]
impl Draw for Label {
    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let s = Size::new(
            if self.text_width > max.width {
                Label::max_width_to(&self.text, max.width)
            } else {
                cmp::max(min.width, self.text_width)
            },
            cmp::max(min.height, 1),
        );
        self.drawable.set_size(&s);
        s
    }

    fn do_draw(&mut self) {
        let w = self.get_rect().get_width();
        let print = if w < self.text_width {
            Print(Label::truncate(&self.text, w))
        } else {
            Print(self.text.clone())
        };

        stdout()
            .queue(self.get_rect().top_left().move_to())
            .unwrap()
            .queue(SetColors(self.colors.clone()))
            .unwrap()
            .queue(print)
            .unwrap();
    }
}
