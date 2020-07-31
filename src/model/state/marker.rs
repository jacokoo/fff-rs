use crate::model::state::publisher::Publisher;
use crate::model::state::{FileVec, MarkerTrait};
use std::cell::RefCell;

pub struct FileMarker {
    marks: Vec<usize>,
    files: FileVec,
    publisher: RefCell<Publisher<Vec<usize>>>,
}

impl FileMarker {
    pub fn new() -> Self {
        FileMarker {
            marks: Vec::new(),
            files: Vec::new(),
            publisher: RefCell::new(Publisher::new()),
        }
    }

    pub fn set_files(&mut self, files: &FileVec) {
        self.files = files.iter().map(|it| it.clone()).collect();
    }

    pub fn subscribe_change<F: Fn(&Vec<usize>) + 'static>(&self, f: F) {
        self.publisher.borrow_mut().subscribe(f)
    }

    fn fire(&self) {
        self.publisher.borrow().notify(&self.marks);
    }
}

impl MarkerTrait for FileMarker {
    fn mark(&mut self, idx: usize) {
        if !self.marks.contains(&idx) {
            self.marks.push(idx);
            self.fire();
        }
    }

    fn unmark(&mut self, idx: usize) {
        if !self.marks.contains(&idx) {
            return;
        }
        if let Some(p) = self.marks.iter().position(|it| it == &idx) {
            self.marks.remove(p);
            self.fire();
        }
    }

    fn is_marked(&self, idx: usize) -> bool {
        self.marks.contains(&idx)
    }

    fn toggle_mark(&mut self, idx: usize) {
        if self.is_marked(idx) {
            self.unmark(idx)
        } else {
            self.mark(idx)
        }
    }

    fn clear_mark(&mut self) {
        self.marks.clear();
        self.fire()
    }

    fn toggle_mark_all(&mut self) {
        if !self.marks.is_empty() {
            self.marks.clear()
        } else {
            self.marks = self.files.iter().enumerate().map(|(i, _)| i).collect();
        }
        self.fire();
    }
}
