use crate::ui::base::draw::{Draw, COUNTER};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crossterm::style::{Colors, Print, SetColors};
use crossterm::QueueableCommand;
use std::borrow::Borrow;
use std::io::stdout;

pub struct Label {
    rect: Rect,
    id: u16,
    text: String,
    drawn: bool,
    text_width: u16,
    colors: Colors,
}

impl Label {
    pub fn new(txt: String) -> Self {
        Label {
            rect: Rect::new(),
            id: COUNTER.next(),
            text_width: Label::width(&txt),
            text: txt,
            drawn: false,
            colors: Colors {
                foreground: None,
                background: None,
            },
        }
    }

    pub fn set_text(&mut self, txt: String) {
        self.text_width = Label::width(&txt);
        self.text = txt;
    }

    pub fn set_color(&mut self, colors: Colors) {
        self.colors = colors;
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

impl Draw for Label {
    fn id(&self) -> u16 {
        self.id
    }

    fn get_rect(&self) -> &Rect {
        &self.rect
    }

    fn move_to(&mut self, point: &Point) {
        self.rect.set_position(point);
    }

    fn ensure(&mut self, size: &Size) -> Size {
        let s = Size::new(
            if self.text_width > size.width {
                Label::max_width_to(&self.text, size.width)
            } else {
                self.text_width
            },
            1,
        );
        self.rect.set_size(&s);
        s
    }

    fn is_drawn(&self) -> bool {
        self.drawn
    }

    fn do_draw(&mut self) {
        let mut w = self.rect.get_width();
        let print = if w < self.text_width {
            Print(Label::truncate(&self.text, w))
        } else {
            Print(self.text.clone())
        };

        stdout()
            .queue(self.rect.top_left().move_to())
            .unwrap()
            .queue(SetColors(self.colors.clone()))
            .unwrap()
            .queue(print)
            .unwrap();

        self.drawn = true;
    }

    fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>> {
        None
    }
}
