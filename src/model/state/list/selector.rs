use crate::common::Publisher;
use crate::model::file::InnerFile;
use crate::model::state::list::{FileVec, SelectorTrait};
use std::rc::Rc;
use std::sync::Arc;

pub struct FileSelector {
    files: FileVec,
    selected: usize,
    publisher: Publisher<usize>,
}

impl FileSelector {
    pub fn new() -> Self {
        FileSelector {
            files: Vec::new(),
            selected: 0,
            publisher: Publisher::new(),
        }
    }

    pub fn set_files(&mut self, fs: &FileVec) {
        self.files = fs.iter().map(|it| it.clone()).collect();
    }

    pub fn subscribe_change<F: Fn(&usize) + 'static + Send + Sync>(&mut self, f: F) {
        self.publisher.subscribe(f)
    }
}

impl SelectorTrait for FileSelector {
    fn selected(&self) -> Option<usize> {
        if self.files.is_empty() {
            None
        } else {
            Some(self.selected)
        }
    }

    fn selected_file(&self) -> Option<Arc<InnerFile>> {
        self.selected().map(|it| self.files[it].clone())
    }

    fn select(&mut self, idx: usize) -> bool {
        if self.files.is_empty() || idx == self.selected {
            return false;
        }

        let mut i = idx;
        if idx >= self.files.len() {
            i = 0;
        }

        if i == self.selected {
            return false;
        }

        self.selected = i;
        self.publisher.notify(&self.selected);
        return true;
    }

    fn move_select(&mut self, delta: i32) -> bool {
        let s = (self.selected as i32) + delta;
        let idx = match s {
            a if a.is_negative() => self.files.len() - (a.wrapping_abs() as usize),
            a if (a as usize) >= self.files.len() => (a as usize) - self.files.len(),
            a => a as usize,
        };

        self.select(idx)
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
