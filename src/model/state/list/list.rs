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
use crate::ui::event::FileItem;
use delegate::delegate;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

pub struct FileList {
    pub dir: Option<InnerFile>,
    filter: FileFilter,
    sorter: Arc<Mutex<FileSorter>>,
    selector: Arc<Mutex<FileSelector>>,
    marker: Arc<Mutex<FileMarker>>,
}

impl FileList {
    pub fn new() -> Self {
        let mut filter = FileFilter::new();
        let sorter = Arc::new(Mutex::new(FileSorter::new(FileSortBy::NAME)));
        let selector = Arc::new(Mutex::new(FileSelector::new()));
        let marker = Arc::new(Mutex::new(FileMarker::new()));

        let s2 = sorter.clone();
        filter.subscribe_change(move |vs| {
            s2.lock().unwrap().set_files(vs);
        });

        let s3 = selector.clone();
        sorter.lock().unwrap().subscribe_change(move |vs| {
            s3.lock().unwrap().set_files(vs);
        });

        let s4 = marker.clone();
        sorter.lock().unwrap().subscribe_change(move |vs| {
            s4.lock().unwrap().set_files(vs);
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
    pub async fn update(&mut self, path: &InnerPath) -> Void {
        let file = InnerFile::try_from(path)?;
        if let InnerFile::Dir(dir) = &file {
            let fs: Vec<_> = dir
                .list()
                .await?
                .into_iter()
                .map(|it| Arc::new(it))
                .collect();
            self.dir = Some(file);
            self.filter.set_files(&fs);
            return Ok(());
        }
        return Err(Error::DirIsRequired(path.to_string()));
    }

    pub fn subscribe_file_change<F: Fn(&FileVec) + 'static + Send + Sync>(&self, f: F) {
        self.sorter.lock().unwrap().subscribe_change(f);
    }

    pub fn subscribe_select_change<F: Fn(&usize) + 'static + Send>(&self, f: F) {
        self.selector.lock().unwrap().subscribe_change(f);
    }

    pub fn subscribe_mark_change<F: Fn(&Vec<usize>) + 'static + Send>(&self, f: F) {
        self.marker.lock().unwrap().subscribe_change(f);
    }

    pub fn file_items(&self) -> Vec<FileItem> {
        self.sorter
            .lock()
            .unwrap()
            .get_files()
            .iter()
            .map(|f| FileItem::from(f.as_ref()))
            .collect()
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
        to self.sorter.lock().unwrap() {
            fn set_order(&mut self, order: FileSortBy);
            fn get_order(&self) -> FileSortBy;
        }
    }
}

impl SelectorTrait for FileList {
    delegate! {
        to self.selector.lock().unwrap() {
            fn selected(&self) -> Option<usize>;
            fn selected_file(&self) -> Option<Arc<InnerFile>>;
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
        to self.marker.lock().unwrap() {
            fn marked(&self) -> Vec<usize>;
            fn mark(&mut self, idx: usize);
            fn unmark(&mut self, idx: usize);
            fn is_marked(&self, idx: usize) -> bool;
            fn toggle_mark(&mut self, idx: usize);
            fn clear_mark(&mut self);
            fn toggle_mark_all(&mut self);
        }
    }
}
