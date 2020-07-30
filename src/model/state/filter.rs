use crate::model::file::path::InnerPath;
use crate::model::file::{FileInfo, InnerFile};
use crate::model::result::{Error, Res, Void};
use crate::model::state::publisher::Publisher;
use crate::model::state::{FileHolder, FileVec};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::ops::Sub;
use std::rc::Rc;
use std::time::SystemTime;
use tokio::time::Duration;

struct FileFilter {
    files: FileVec,
    filtered: FileVec,
    filter: Filter,
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
            filter: Filter::new(),
            show_detail: false,
            publisher: RefCell::new(Publisher::new()),
        }
    }

    pub fn set_files(&mut self, files: &FileVec) {
        self.files = files.iter().map(|f| f.clone()).collect();
        self.do_filter();
    }

    pub fn set_filter(&mut self, filter: String) -> Void {
        self.filter.update(&filter)?;
        self.do_filter();

        Ok(())
    }

    pub fn set_show_detail(&mut self, show: bool) {
        let old = self.show_detail;
        self.show_detail = show;

        if old != self.show_detail {
            self.filter.show_hidden(self.show_detail);
            self.do_filter();
        }
    }

    fn do_filter(&mut self) {
        self.filtered = self
            .files
            .iter()
            .filter(|it| self.filter.matches(it.info()))
            .map(|it| it.clone())
            .collect();
        self.publisher.borrow().notify(&self.filtered);
    }
}

create_enum!(TimeUnit: H, D, M);
create_enum!(SizeUnit: K, M, G);

impl TimeUnit {
    fn to_seconds(&self, n: &u64) -> u64 {
        match self {
            Self::H => n * 60 * 60,
            Self::D => n * 24 * 60 * 60,
            Self::M => n * 30 * 24 * 60 * 60,
        }
    }
}

impl SizeUnit {
    fn to_bytes(&self, n: &u64) -> u64 {
        match self {
            Self::K => n * 1024,
            Self::M => n * 1024 * 1024,
            Self::G => n * 1024 * 1024 * 1024,
        }
    }
}

enum FilterItem {
    None,
    NoHidden,
    Type(bool), // true is file, false is dir
    MTime(u64, TimeUnit),
    Size(bool, u64, SizeUnit), // true is >, false is <
    Name(String),
}

impl TryFrom<&str> for FilterItem {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = &value[0..1];
        if s == ":" {
            return match &value[1..] {
                "f" => Ok(FilterItem::Type(true)),
                "d" => Ok(FilterItem::Type(false)),
                _ => Err(Error::InvalidFilter(value.to_string())),
            };
        }

        if s == "+" {
            let u = TimeUnit::try_from(&value[value.len() - 1..])?;
            let s = (&value[1..value.len() - 1]).parse::<u64>();
            return match s {
                Ok(ss) => Ok(FilterItem::MTime(ss, u)),
                Err(_) => Err(Error::InvalidFilter(value.to_string())),
            };
        }

        if s == ">" || s == "<" {
            let u = SizeUnit::try_from(&value[value.len() - 1..])?;
            let n = (&value[1..value.len() - 1]).parse::<u64>();
            return match n {
                Ok(ss) => Ok(FilterItem::Size(s == ">", ss, u)),
                Err(_) => Err(Error::InvalidFilter(value.to_string())),
            };
        }

        return Ok(FilterItem::Name(value.to_string()));
    }
}

impl FilterItem {
    fn matches(&self, fi: &FileInfo) -> bool {
        match self {
            Self::None => true,
            Self::NoHidden => !fi.name.starts_with("."),
            Self::Name(s) => fi.name.contains(s),
            Self::Type(s) => {
                if s.clone() {
                    !fi.is_dir
                } else {
                    fi.is_dir
                }
            }
            Self::Size(gt, n, u) => {
                let ss = SizeUnit::to_bytes(u, n);
                if gt.clone() {
                    fi.size > ss
                } else {
                    fi.size < ss
                }
            }
            Self::MTime(n, u) => {
                let sec = Duration::from_secs(TimeUnit::to_seconds(u, n));
                let now = SystemTime::now().sub(sec);

                match fi.modified {
                    None => true,
                    Some(time) => time > now,
                }
            }
        }
    }
}

struct Filter(FilterItem, Vec<FilterItem>);

impl Filter {
    fn new() -> Self {
        Filter(FilterItem::None, Vec::new())
    }

    fn parse(value: &str) -> Res<Vec<FilterItem>> {
        let v = value.trim();
        if v.len() == 0 {
            return Ok(vec![FilterItem::None]);
        }

        let vs = value
            .split(" ")
            .filter(|it| it.len() > 0)
            .map(|it| FilterItem::try_from(it))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(vs);
    }

    fn show_hidden(&mut self, show: bool) {
        self.0 = if show {
            FilterItem::None
        } else {
            FilterItem::NoHidden
        }
    }

    fn update(&mut self, str: &str) -> Void {
        self.1 = Filter::parse(str)?;
        Ok(())
    }

    fn matches(&self, fi: &FileInfo) -> bool {
        self.0.matches(fi) && self.1.iter().all(|it| it.matches(fi))
    }
}
