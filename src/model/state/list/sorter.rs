use crate::common::Publisher;
use crate::model::state::list::{FileHolder, FileSortBy, FileVec, SorterTrait};
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct FileSorter {
    files: FileVec,
    sorted: FileVec,
    order: FileSortBy,
    publisher: Publisher<FileVec>,
}

impl FileHolder for FileSorter {
    fn get_files(&self) -> &FileVec {
        &self.sorted
    }

    fn subscribe_change<F: Fn(&FileVec) + 'static + Send + Sync>(&mut self, f: F) {
        self.publisher.subscribe(f);
    }
}

impl FileSorter {
    pub fn new(order: FileSortBy) -> Self {
        FileSorter {
            files: Vec::new(),
            sorted: Vec::new(),
            order,
            publisher: Publisher::new(),
        }
    }

    pub fn set_files(&mut self, files: &FileVec) {
        self.files = files.iter().map(|it| it.clone()).collect();
        self.do_sort();
    }

    fn do_sort(&mut self) {
        self.sorted = self.files.iter().map(|it| it.clone()).collect();
        let order = &self.order;
        self.sorted.sort_by(|a, b| {
            if a.is_dir() && b.is_file() {
                return Ordering::Less;
            }

            if a.is_file() && b.is_dir() {
                return Ordering::Greater;
            }

            return match order {
                FileSortBy::NAME => a.info().name.cmp(&b.info().name),
                FileSortBy::SIZE => b.info().size.cmp(&a.info().size),
                FileSortBy::MTIME => b.info().modified.cmp(&a.info().modified),
            };
        });

        self.publisher.notify(&self.sorted);
    }
}

impl SorterTrait for FileSorter {
    fn set_order(&mut self, order: FileSortBy) {
        if order == self.order {
            return;
        }

        self.order = order;
        self.do_sort();
    }

    fn get_order(&self) -> FileSortBy {
        self.order.clone()
    }
}
