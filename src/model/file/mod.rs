use crate::model::context::Context;
use crate::model::file::path::InnerPath;
use crate::model::result::{Error, Res, Void};
use async_trait::async_trait;
use chrono::{DateTime, Local};
pub use local::make;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

mod cmd;
mod file_mode;
mod local;
pub mod path;
mod protocol;

pub struct FileInfo {
    pub inner: InnerPath,
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub mode: String,
    pub modified: Option<SystemTime>,
    pub is_dir: bool,
    pub link: Option<LinkInfo>,
    pub protocol: Option<ProtocolInfo>,
}

pub struct LinkInfo {
    pub broken: bool,
    pub target: String,
}

pub struct ProtocolInfo {
    pub protocol: String,
    pub instance_id: u8,
    pub root: Box<Arc<FileInfo>>,
}

pub enum InnerFile {
    File(Box<dyn FileOp + Send + Sync>),
    Dir(Box<dyn DirOp + Send + Sync>),
}

impl InnerFile {
    pub fn info(&self) -> &FileInfo {
        match self {
            InnerFile::File(file) => file.get(),
            InnerFile::Dir(dir) => dir.get(),
        }
    }

    pub fn is_dir(&self) -> bool {
        if let InnerFile::Dir(_) = self {
            return true;
        }
        return false;
    }

    pub fn is_file(&self) -> bool {
        return !self.is_dir();
    }

    pub fn path_str(&self) -> String {
        self.inner_path().to_string()
    }

    pub fn inner_path(&self) -> &InnerPath {
        &self.info().inner
    }

    pub fn readable_size(&self) -> String {
        let info = self.info();
        let mut unit = "B";
        let base = 1024f64;
        let mut size = info.size as f64;

        if size > base {
            unit = "K";
            size = size / base;
        } else {
            return format!("{}{}", info.size, unit);
        }

        if size > base {
            unit = "M";
            size = size / base;
        }

        if size > base {
            unit = "G";
            size = size / base;
        }

        return format!("{0:.2}{1}", size, unit);
    }

    pub fn modify_time_str(&self) -> String {
        let info = self.info();
        match &info.modified {
            Some(v) => {
                let dt: DateTime<Local> = v.clone().into();
                dt.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            None => format!("{0:^19}", "-"),
        }
    }
}

impl TryFrom<InnerPath> for InnerFile {
    type Error = Error;

    fn try_from(value: InnerPath) -> Res<Self> {
        // TODO check protocol
        return make(value);
    }
}

#[async_trait]
impl Op for InnerFile {
    fn get(&self) -> &FileInfo {
        match self {
            InnerFile::File(v) => v.get(),
            InnerFile::Dir(v) => v.get(),
        }
    }

    async fn parent(&self, context: &Context) -> Res<InnerFile> {
        match self {
            InnerFile::File(v) => v.parent(context),
            InnerFile::Dir(v) => v.parent(context),
        }
        .await
    }

    async fn rename(&self, context: &Context) -> Void {
        match self {
            InnerFile::File(v) => v.rename(context),
            InnerFile::Dir(v) => v.rename(context),
        }
        .await
    }

    async fn delete(&self, context: &Context) -> Void {
        match self {
            InnerFile::File(v) => v.delete(context),
            InnerFile::Dir(v) => v.delete(context),
        }
        .await
    }

    async fn open(&self, context: &Context) -> Void {
        match self {
            InnerFile::File(v) => v.open(context),
            InnerFile::Dir(v) => v.open(context),
        }
        .await
    }
}

// common file operators
#[async_trait]
pub trait Op {
    fn get(&self) -> &FileInfo;
    async fn parent(&self, context: &Context) -> Res<InnerFile>;
    async fn rename(&self, context: &Context) -> Void;
    async fn delete(&self, context: &Context) -> Void;
    async fn open(&self, context: &Context) -> Void;
}

#[async_trait]
pub trait FileOp: Op {
    async fn view(&self, context: &Context) -> Void;
    async fn edit(&self, context: &Context) -> Void;
}

#[async_trait]
pub trait DirOp: Op {
    async fn list(&self, context: &Context) -> Res<Vec<InnerFile>>;
    async fn new_file(&self, context: &Context) -> Void;
    async fn new_dir(&self, context: &Context) -> Void;
    async fn goto(&self, context: &Context, child_path: &str) -> Res<InnerFile>;
    async fn shell(&self, context: &Context) -> Void;
}
