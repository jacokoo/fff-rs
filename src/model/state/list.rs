use crate::model::file::InnerFile;
use crate::model::state::Order;
use std::cmp::Ordering;
use std::rc::Rc;

struct FileHolder {
    files: Vec<Rc<InnerFile>>,
}

pub struct FileList {
    show_detail: bool,
    order: Order,
    files: Vec<InnerFile>,
}

impl FileList {
    pub fn sort(&mut self, order: Order) {
        self.files.sort_by(|a, b| {
            if a.is_dir() && b.is_file() {
                return Ordering::Less;
            }

            if a.is_file() && b.is_dir() {
                return Ordering::Greater;
            }

            return match order {
                Order::ByName => b.info().name.cmp(&a.info().name),
                Order::BySize => b.info().size.cmp(&a.info().size),
                Order::ByLastModified => b.info().modified.cmp(&a.info().modified),
            };
        });

        self.order = order;
    }

    pub fn show_detail(&mut self, show: bool) {
        self.show_detail = show;
    }

    pub fn is_show_detail(&self) -> bool {
        return self.show_detail;
    }
}
