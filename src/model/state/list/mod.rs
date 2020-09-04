use crate::model::file::InnerFile;
use crate::model::result::Void;
use crate::ui::event::FileItem;
use std::rc::Rc;

mod filter;
pub mod list;
mod marker;
mod selector;
mod sorter;

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

pub trait FilterTrait {
    fn is_show_hidden(&self) -> bool;
    fn set_filter(&mut self, str: String) -> Void;
    fn toggle_show_hidden(&mut self);
    fn set_show_hidden(&mut self, show: bool);
}

pub trait SorterTrait {
    fn set_order(&mut self, order: FileSortBy);
    fn get_order(&self) -> FileSortBy;
}

pub trait SelectorTrait {
    fn selected(&self) -> Option<usize>;
    fn selected_file(&self) -> Option<Rc<InnerFile>>;
    fn select(&mut self, idx: usize) -> bool;
    fn move_select(&mut self, delta: i32) -> bool;
    fn select_by_name(&mut self, name: &str) -> bool;
    fn select_first(&mut self) -> bool;
    fn select_last(&mut self) -> bool;
}

pub trait MarkerTrait {
    fn marked(&self) -> Vec<usize>;
    fn mark(&mut self, idx: usize);
    fn unmark(&mut self, idx: usize);
    fn is_marked(&self, idx: usize) -> bool;
    fn toggle_mark(&mut self, idx: usize);
    fn clear_mark(&mut self);
    fn toggle_mark_all(&mut self);
}
