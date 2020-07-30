use crate::model::file::path::InnerPath;
use crate::model::file::InnerFile;
use crate::model::result::{Error, Void};
use crate::model::state::publisher::Publisher;
use crate::model::state::{FileHolder, FileVec};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

struct FileFilter {
    files: FileVec,
    filtered: FileVec,
    filter: String,
    show_detail: bool,
    publisher: RefCell<Publisher<FileVec>>,
}

impl FileHolder for FileFilter {
    fn get_files(&self) -> &FileVec {
        return &self.filtered;
    }

    fn subscribe_change<F: Fn(&FileVec) + 'static>(&self, f: F) {
        self.publisher.borrow_mut().subscribe(f);
    }
}

impl FileFilter {
    pub fn new() -> Self {
        FileFilter {
            files: Vec::new(),
            filtered: Vec::new(),
            filter: "".to_string(),
            show_detail: false,
            publisher: RefCell::new(Publisher::new()),
        }
    }

    pub fn set_files(&mut self, files: &FileVec) {
        self.files = files.iter().map(|f| f.clone()).collect();
        self.do_filter();
    }

    pub fn set_filter(&mut self, filter: String) -> Void {
        self.filter = filter;
        self.do_filter();

        Ok(())
    }

    pub fn set_show_detail(&mut self, show: bool) {
        let old = self.show_detail;
        self.show_detail = show;

        if old != self.show_detail {
            self.do_filter();
        }
    }

    fn do_filter(&mut self) {
        self.publisher.borrow().notify(&self.filtered);
    }
}

create_enum!(TimeUnit: H, D, M);
create_enum!(SizeUnit: K, M, G);

enum FilterItem {
    None,
    Type(bool), // true is file, false is dir
    MTime(u8, TimeUnit),
    Size(bool, u64, SizeUnit),
    Name(String),
}

struct Filter {
    items: Vec<FilterItem>,
}

impl TryFrom<&str> for Filter {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
