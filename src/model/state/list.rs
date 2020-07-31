use crate::model::file::make;
use crate::model::file::path::InnerPath;
use crate::model::state::filter::FileFilter;
use crate::model::state::sorter::{FileSortBy, FileSorter};
use crate::model::state::FileHolder;
use std::cell::RefCell;
use std::rc::Rc;

struct FileList {
    path: InnerPath,
    filter: FileFilter,
    sorter: Rc<RefCell<FileSorter>>,
}

impl FileList {
    fn new(path: InnerPath) -> Self {
        let filter = FileFilter::new();
        let sorter = Rc::new(RefCell::new(FileSorter::new(FileSortBy::NAME)));

        let s2 = sorter.clone();
        filter.subscribe_change(move |vs| {
            s2.borrow_mut().set_files(vs);
        });

        FileList {
            path,
            filter,
            sorter,
        }
    }
}
