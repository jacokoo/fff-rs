use crate::model::file::InnerFile;
use crate::model::result::Void;
use std::rc::Rc;

mod filter;
pub mod list;
mod marker;
mod selector;
mod sorter;
mod workspace;

pub type FileVec = Vec<Rc<InnerFile>>;

#[derive(PartialEq, PartialOrd, Clone)]
pub enum FileSortBy {
    NAME,
    MTIME,
    SIZE,
}

trait FileHolder {
    fn get_files(&self) -> &FileVec;
    fn subscribe_change<F: Fn(&FileVec) + 'static>(&self, f: F);
}

trait FilterTrait {
    fn is_show_hidden(&self) -> bool;
    fn set_filter(&mut self, str: String) -> Void;
    fn toggle_show_hidden(&mut self);
    fn set_show_hidden(&mut self, show: bool);
}

trait SorterTrait {
    fn set_order(&mut self, order: FileSortBy);
    fn get_order(&self) -> FileSortBy;
}

trait SelectorTrait {
    fn selected_file(&self) -> Option<Rc<InnerFile>>;
    fn select(&mut self, idx: usize) -> bool;
    fn move_select(&mut self, delta: i32) -> bool;
    fn select_by_name(&mut self, name: &str) -> bool;
    fn select_first(&mut self) -> bool;
    fn select_last(&mut self) -> bool;
}

trait MarkerTrait {
    fn mark(&mut self, idx: usize);
    fn unmark(&mut self, idx: usize);
    fn is_marked(&self, idx: usize) -> bool;
    fn toggle_mark(&mut self, idx: usize);
    fn clear_mark(&mut self);
    fn toggle_mark_all(&mut self);
}
