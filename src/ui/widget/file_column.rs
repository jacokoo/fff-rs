use crate::model::state::workspace::ViewMode;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::Size;
use crate::ui::event::FileItem;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::corner_line::CornerLine;
use crate::ui::widget::file_list::FileList;
use crate::ui::{Mrc, ToMrc};
use std::cell::{Ref, RefMut};

pub struct FileColumn {
    columns: Vec<Mrc<FileList>>,
    lines: Vec<Mrc<SizedBox>>,
    flex: Flex,
    show_detail: bool,
    mode: ViewMode,
}

impl FileColumn {
    pub fn new() -> Self {
        FileColumn {
            columns: Vec::new(),
            lines: Vec::new(),
            flex: Flex::row(),
            show_detail: false,
            mode: ViewMode::InColumn,
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

    pub fn add_file_list(&mut self, files: Vec<FileItem>) {
        if let ViewMode::InList = self.mode {
            self.current_mut().set_files(files);
            return;
        }

        if self.show_detail {
            self.current_mut().set_show_detail(false);
        }

        let mut ls = FileList::new(self.show_detail);
        ls.set_files(files);

        self.columns.push(ls.mrc());
        self.add_line();
    }

    pub fn remove_file_list(&mut self, files: Option<Vec<FileItem>>) {
        if self.columns.len() > 1 {
            self.columns.pop();
            return;
        }

        self.current_mut().set_files(files.unwrap());
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
