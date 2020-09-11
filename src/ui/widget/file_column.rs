use crate::model::state::workspace::ViewMode;
use crate::ui::base::draw::Draw;
use crate::ui::event::FileItem;
use crate::ui::layout::flex::Flex;
use crate::ui::widget::file_list::FileList;
use crate::ui::{Mrc, ToMrc};
use std::cell::{Ref, RefMut};

pub struct FileColumn {
    columns: Vec<Mrc<FileList>>,
    flex: Flex,
    show_detail: bool,
    mode: ViewMode,
}

impl FileColumn {
    pub fn new() -> Self {
        FileColumn {
            columns: Vec::new(),
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
        self.current_mut().clear();
        self.current_mut().set_show_detail(show);
        self.current_mut().redraw();
    }

    pub fn current(&self) -> Ref<FileList> {
        self.columns.last().unwrap().borrow()
    }

    pub fn current_mut(&self) -> RefMut<FileList> {
        self.columns.last().unwrap().borrow_mut()
    }

    pub fn init_file_list(&mut self, lists: Vec<Vec<FileItem>>) {
        if self.columns.len() > lists.len() {
            for _ in 0..(self.columns.len() - lists.len()) {
                self.columns.pop();
                self.flex.pop();
            }
        }

        if self.columns.len() < lists.len() {
            for i in 0..(lists.len() - self.columns.len()) {
                let fl = FileList::new(self.show_detail && (i == lists.len() - 1)).mrc();
                self.columns.push(fl.clone());
                self.flex.add(fl);
            }
        }

        for (idx, it) in lists.into_iter().enumerate() {
            self.columns[idx].borrow_mut().set_files(it);
        }

        self.redraw();
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
            self.current_mut().redraw();
        }

        let mut ls = FileList::new(self.show_detail);
        ls.set_files(files);
        let fl = ls.mrc();

        self.columns.push(fl.clone());
        self.flex.push(fl);
    }

    pub fn remove_file_list(&mut self, files: Option<Vec<FileItem>>) {
        if self.columns.len() > 1 {
            self.columns.pop();
            self.flex.pop();
            if self.show_detail {
                self.current_mut().set_show_detail(true);
                self.current_mut().redraw();
            }
            return;
        }

        self.current_mut().set_files(files.unwrap());
    }

    pub fn keep_last(&mut self) {
        let c = self.columns.last();

        if let Some(cc) = c {
            let lc = cc.clone();
            self.columns.clear();
            self.flex.clear();
            self.add_item(lc);
        }

        self.redraw();
    }

    fn add_item(&mut self, item: Mrc<FileList>) {
        self.flex.add(item.clone());
        self.columns.push(item);
    }
}

#[draw_to(flex)]
impl Draw for FileColumn {}
