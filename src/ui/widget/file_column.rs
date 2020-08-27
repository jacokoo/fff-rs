use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::file_list::FileList;
use crate::ui::widget::label::Label;
use crate::ui::widget::line::Line;
use crate::ui::{Mrc, ToMrc};
use delegate::delegate;
use std::cell::RefMut;
use std::ops::Deref;

pub struct CornerLine {
    line: Line,
    label: Label,
    corner_char: char,
    clear_char: char,
}

impl CornerLine {
    pub fn new(line_char: char, corner_char: char, clear_char: char) -> Self {
        CornerLine {
            line: Line::new_vertical(line_char),
            label: Label::from(corner_char.to_string()),
            corner_char,
            clear_char,
        }
    }
}

impl Draw for CornerLine {
    delegate! {
        to self.line {
            fn get_rect(&self) -> &Rect;
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.label.ensure(min, max);
        return self.line.ensure(min, max);
    }

    fn move_to(&mut self, point: &Point) {
        self.line.move_to(point);
        self.label.move_to(&(point + (0i16, -1i16)));
    }

    fn do_draw(&mut self) {
        self.line.draw();
        self.label.draw();
    }

    fn clear(&mut self) {
        self.line.clear();
        self.label.set_text(self.clear_char.to_string());
        self.label.draw();
        self.label.set_text(self.corner_char.to_string());
    }
}

pub struct FileColumn {
    columns: Vec<Mrc<FileList>>,
    lines: Vec<Mrc<SizedBox>>,
    flex: Flex,
}

impl FileColumn {
    pub fn new() -> Self {
        FileColumn {
            columns: Vec::new(),
            lines: Vec::new(),
            flex: Flex::row(),
        }
    }

    pub fn add_file_list(&mut self, list: Mrc<FileList>) {
        self.columns.push(list.clone());
        self.flex.add(list);
        self.add_line();
    }

    pub fn keep_last(&mut self) {
        let c = self.columns.last();
        let l = self.lines.last();

        if let Some(cc) = c {
            let lc = cc.clone();
            let ll = l.unwrap().clone();

            self.lines.clear();
            self.columns.clear();
            self.columns.push(lc.clone());
            self.lines.push(ll.clone());
            self.flex.add(lc);
            self.flex.add(ll);
        }
    }

    pub fn last(&self) -> Option<RefMut<FileList>> {
        if let Some(v) = self.columns.last() {
            Some(v.deref().borrow_mut())
        } else {
            None
        }
    }

    fn add_line(&mut self) {
        let line = SizedBox::new(CornerLine::new('│', '┬', '─').mrc())
            .max_height()
            .mrc();
        self.lines.push(line.clone());
        self.flex.add(line);
    }
}

impl Draw for FileColumn {
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
