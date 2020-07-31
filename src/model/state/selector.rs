use crate::model::file::InnerFile;
use crate::model::state::publisher::Publisher;
use crate::model::state::{FileVec, SelectorTrait};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FileSelector {
    files: FileVec,
    selected: usize,
    publisher: RefCell<Publisher<usize>>,
}

impl FileSelector {
    pub fn new() -> Self {
        FileSelector {
            files: Vec::new(),
            selected: 0,
            publisher: RefCell::new(Publisher::new()),
        }
    }

    pub fn set_files(&mut self, fs: &FileVec) {
        self.files = fs.iter().map(|it| it.clone()).collect();
    }

    pub fn subscribe_change<F: Fn(&usize) + 'static>(&self, f: F) {
        self.publisher.borrow_mut().subscribe(f)
    }
}

impl SelectorTrait for FileSelector {
    fn selected_file(&self) -> Option<Rc<InnerFile>> {
        if self.files.is_empty() {
            None
        } else {
            Some(self.files[self.selected].clone())
        }
    }

    fn select(&mut self, idx: usize) -> bool {
        if self.files.is_empty() || idx == self.selected {
            return false;
        }

        let mut i = idx;
        if idx >= self.files.len() {
            i = 0;
        }

        self.selected = i;
        if i == self.selected {
            return false;
        }

        self.publisher.borrow_mut().notify(&self.selected);
        return true;
    }

    fn move_select(&mut self, delta: i32) -> bool {
        if delta.is_negative() {
            self.select(self.selected - delta.wrapping_abs() as usize)
        } else {
            self.select(self.selected + delta as usize)
        }
    }

    fn select_by_name(&mut self, name: &str) -> bool {
        if let Some(idx) = self.files.iter().position(|it| it.info().name == name) {
            return self.select(idx);
        }
        return false;
    }

    fn select_first(&mut self) -> bool {
        self.select(0)
    }

    fn select_last(&mut self) -> bool {
        self.select(self.files.len() - 1)
    }
}
