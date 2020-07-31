use crate::model::file::path::InnerPath;
use crate::model::file::{InnerFile};
use crate::model::result::{Error, Void};
use crate::model::state::filter::FileFilter;
use crate::model::state::marker::FileMarker;
use crate::model::state::selector::FileSelector;
use crate::model::state::sorter::{FileSortBy, FileSorter};
use crate::model::state::FileHolder;
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
        sorter.borrow().subscribe_change(move |vs| {
            s3.borrow_mut().set_files(vs);
        });

        let s4 = marker.clone();
        sorter.borrow().subscribe_change(move |vs| {
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
    async fn update(&mut self, path: InnerPath) -> Void {
        if let InnerFile::Dir(_dir) = InnerFile::try_from(&path)? {}
        return Err(Error::DirIsRequired(path.to_string()));
    }
}
