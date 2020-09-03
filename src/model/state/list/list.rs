use crate::model::file::path::InnerPath;
use crate::model::file::InnerFile;
use crate::model::result::{Error, Void};
use crate::model::state::list::filter::FileFilter;
use crate::model::state::list::marker::FileMarker;
use crate::model::state::list::selector::FileSelector;
use crate::model::state::list::sorter::FileSorter;
use crate::model::state::list::{
    FileHolder, FileSortBy, FileVec, FilterTrait, MarkerTrait, SelectorTrait, SorterTrait,
};
use delegate::delegate;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

pub struct FileList {
    dir: Option<InnerFile>,
    filter: FileFilter,
    sorter: Rc<RefCell<FileSorter>>,
    selector: Rc<RefCell<FileSelector>>,
    marker: Rc<RefCell<FileMarker>>,
}

impl FileList {
    pub fn new() -> Self {
        let filter = FileFilter::new();
        let sorter = Rc::new(RefCell::new(FileSorter::new(FileSortBy::NAME)));
        let selector = Rc::new(RefCell::new(FileSelector::new()));
        let marker = Rc::new(RefCell::new(FileMarker::new()));

        let s2 = sorter.clone();
        filter.subscribe_change(move |vs| {
            s2.borrow_mut().set_files(vs);
        });

        let s3 = selector.clone();
        sorter.borrow_mut().subscribe_change(move |vs| {
            s3.borrow_mut().set_files(vs);
        });

        let s4 = marker.clone();
        sorter.borrow_mut().subscribe_change(move |vs| {
            s4.borrow_mut().set_files(vs);
        });

        FileList {
            dir: None,
            filter,
            sorter,
            selector,
            marker,
        }
    }
}

impl FileList {
    pub async fn update(&mut self, path: InnerPath) -> Void {
        let file = InnerFile::try_from(&path)?;
        if let InnerFile::Dir(dir) = &file {
            let fs: Vec<_> = dir
                .list()
                .await?
                .into_iter()
                .map(|it| Rc::new(it))
                .collect();
            self.dir = Some(file);
            self.filter.set_files(&fs);
        }
        return Err(Error::DirIsRequired(path.to_string()));
    }

    pub fn subscribe_file_change<F: Fn(&FileVec) + 'static>(&self, f: F) {
        self.sorter.borrow_mut().subscribe_change(f);
    }

    pub fn subscribe_select_change<F: Fn(&usize) + 'static>(&self, f: F) {
        self.selector.borrow_mut().subscribe_change(f);
    }

    pub fn subscribe_mark_change<F: Fn(&Vec<usize>) + 'static>(&self, f: F) {
        self.marker.borrow_mut().subscribe_change(f);
    }
}

impl FilterTrait for FileList {
    delegate! {
        to self.filter {
            fn is_show_hidden(&self) -> bool;
            fn set_filter(&mut self, str: String) -> Void;
            fn toggle_show_hidden(&mut self);
            fn set_show_hidden(&mut self, show: bool);
        }
    }
}

impl SorterTrait for FileList {
    delegate! {
        to self.sorter.borrow_mut() {
            fn set_order(&mut self, order: FileSortBy);
            fn get_order(&self) -> FileSortBy;
        }
    }
}

impl SelectorTrait for FileList {
    delegate! {
        to self.selector.borrow_mut() {
            fn selected_file(&self) -> Option<Rc<InnerFile>>;
            fn select(&mut self, idx: usize) -> bool;
            fn move_select(&mut self, delta: i32) -> bool;
            fn select_by_name(&mut self, name: &str) -> bool;
            fn select_first(&mut self) -> bool;
            fn select_last(&mut self) -> bool;
        }
    }
}

impl MarkerTrait for FileList {
    delegate! {
        to self.marker.borrow_mut() {
            fn mark(&mut self, idx: usize);
            fn unmark(&mut self, idx: usize);
            fn is_marked(&self, idx: usize) -> bool;
            fn toggle_mark(&mut self, idx: usize);
            fn clear_mark(&mut self);
            fn toggle_mark_all(&mut self);
        }
    }
}
