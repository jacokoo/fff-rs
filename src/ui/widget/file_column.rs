use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::event::FileItem;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::file_list::FileList;
use crate::ui::widget::label::Label;
use crate::ui::widget::line::Line;
use crate::ui::{Mrc, ToMrc};
use std::cell::{Ref, RefMut};

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

#[draw_to(line)]
impl Draw for CornerLine {
    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.label.ensure(min, max);
        return self.line.ensure(min, max);
    }

    fn move_to(&mut self, point: &Point) {
        self.line.move_to(point);
        self.label.move_to(&(point + (0, -1)));
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
    show_detail: bool,
}

impl FileColumn {
    pub fn new() -> Self {
        FileColumn {
            columns: Vec::new(),
            lines: Vec::new(),
            flex: Flex::row(),
            show_detail: false,
        }
    }

    pub fn set_show_detail(&mut self, show: bool) {
        if self.show_detail == show {
            return;
        }

        self.show_detail = show;
        self.current_mut().set_show_detail(show);
    }

    pub fn current(&self) -> Ref<FileList> {
        self.columns.last().unwrap().borrow()
    }

    pub fn current_mut(&self) -> RefMut<FileList> {
        self.columns.last().unwrap().borrow_mut()
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.columns.clear();
        self.flex.empty_it();
    }

    pub fn init_file_list(&mut self, lists: Vec<Vec<FileItem>>) {
        if self.columns.len() > lists.len() {
            for _ in 0..(self.columns.len() - lists.len()) {
                self.columns.pop();
                self.lines.pop();
            }
        }

        if self.columns.len() < lists.len() {
            for i in 0..(lists.len() - self.columns.len()) {
                self.columns
                    .push(FileList::new(self.show_detail && (i == lists.len() - 1)).mrc());
                self.add_line();
            }
        }

        for (idx, it) in lists.into_iter().enumerate() {
            self.columns[idx].borrow_mut().set_files(it);
        }
    }

    pub fn init_selected(&mut self, selected: Vec<Option<usize>>) {
        for (idx, it) in self.columns.iter().enumerate() {
            it.borrow_mut().set_selected(selected[idx]);
        }
    }

    pub fn init_marked(&mut self, marks: Vec<Vec<usize>>) {
        for (idx, it) in marks.into_iter().enumerate() {
            self.columns[idx].borrow_mut().set_marked(it);
        }
    }

    pub fn add_file_list(&mut self, list: Mrc<FileList>) {
        self.columns.push(list.clone());
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
            self.columns.push(lc);
            self.lines.push(ll);
        }
    }

    fn add_line(&mut self) {
        let line = SizedBox::new(CornerLine::new('│', '┬', '─').mrc())
            .max_height()
            .mrc();
        self.lines.push(line.clone());
    }

    fn prepare_ensure(&mut self) {
        self.flex.empty_it();
        for (idx, it) in self.columns.iter().enumerate() {
            if (idx == self.columns.len() - 1) && self.show_detail {
                self.flex.add(it.clone());
            } else {
                self.flex.add(SizedBox::new(it.clone()).width(30).mrc());
            }

            self.flex.add(self.lines[idx].clone());
        }
    }
}

#[draw_to(flex)]
impl Draw for FileColumn {
    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.prepare_ensure();
        self.flex.ensure(min, max)
    }
}
