use crate::model::file::InnerFile;
use std::rc::Rc;

mod filter;
mod list;
mod marker;
pub mod publisher;
mod selector;
mod sorter;

pub enum Order {
    ByName,
    ByLastModified,
    BySize,
}

pub type FileVec = Vec<Rc<InnerFile>>;

trait FileHolder {
    fn get_files(&self) -> &FileVec;
    fn subscribe_change<F: Fn(&FileVec) + 'static>(&self, f: F);
}
