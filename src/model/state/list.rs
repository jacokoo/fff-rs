use crate::model::file::path::InnerPath;
use crate::model::file::InnerFile;
use crate::model::result::{Error, Void};
use crate::model::state::filter::FileFilter;
use crate::model::state::marker::FileMarker;
use crate::model::state::selector::FileSelector;
use crate::model::state::sorter::FileSorter;
use crate::model::state::{
    FileHolder, FileSortBy, FileVec, FilterTrait, MarkerTrait, SelectorTrait, SorterTrait,
};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

pub struct FileList {
    file: Option<InnerFile>,
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
            file: None,
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
            self.file = Some(file);
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
    fn is_show_detail(&self) -> bool {
        self.filter.is_show_detail()
    }

    fn set_filter(&mut self, str: String) -> Void {
        self.filter.set_filter(str)
    }

    fn toggle_show_detail(&mut self) {
        self.filter.set_show_detail(!self.filter.is_show_detail())
    }

    fn set_show_detail(&mut self, show: bool) {
        self.filter.set_show_detail(show)
    }
}

impl SorterTrait for FileList {
    fn set_order(&mut self, order: FileSortBy) {
        self.sorter.borrow_mut().set_order(order)
    }

    fn get_order(&self) -> FileSortBy {
        self.sorter.borrow_mut().get_order()
    }
}

impl SelectorTrait for FileList {
    fn selected_file(&self) -> Option<Rc<InnerFile>> {
        self.selector.borrow_mut().selected_file()
    }

    fn select(&mut self, idx: usize) -> bool {
        self.selector.borrow_mut().select(idx)
    }

    fn move_select(&mut self, delta: i32) -> bool {
        self.selector.borrow_mut().move_select(delta)
    }

    fn select_by_name(&mut self, name: &str) -> bool {
        self.selector.borrow_mut().select_by_name(name)
    }

    fn select_first(&mut self) -> bool {
        self.selector.borrow_mut().select_first()
    }

    fn select_last(&mut self) -> bool {
        self.selector.borrow_mut().select_last()
    }
}

impl MarkerTrait for FileList {
    fn mark(&mut self, idx: usize) {
        self.marker.borrow_mut().mark(idx)
    }

    fn unmark(&mut self, idx: usize) {
        self.marker.borrow_mut().unmark(idx)
    }

    fn is_marked(&self, idx: usize) -> bool {
        self.marker.borrow_mut().is_marked(idx)
    }

    fn toggle_mark(&mut self, idx: usize) {
        self.marker.borrow_mut().toggle_mark(idx)
    }

    fn clear_mark(&mut self) {
        self.marker.borrow_mut().clear_mark()
    }

    fn toggle_mark_all(&mut self) {
        self.marker.borrow_mut().toggle_mark_all()
    }
}
