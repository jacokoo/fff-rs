use std::convert::TryFrom;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;

pub use local::make;

use crate::model::file::path::InnerPath;
use crate::model::result::{Error, Res, Void};

pub mod path;

mod cmd;
mod file_mode;
mod local;
mod protocol;

pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub mode: String,
    pub modified: Option<SystemTime>,
    pub is_dir: bool,
    pub link: Option<LinkInfo>,
    pub protocol: Option<ProtocolInfo>,
}

impl FileInfo {
    pub fn get_path(&self) -> &Path {
        Path::new(&self.path)
    }
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
    File(Box<dyn FileOp>),
    Dir(Box<dyn DirOp>),
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
}

impl TryFrom<InnerPath> for InnerFile {
    type Error = Error;

    fn try_from(value: InnerPath) -> Res<Self> {
        // TODO check protocol
        return make(Path::new(&value.path));
    }
}

impl TryFrom<String> for InnerFile {
    type Error = Error;

    fn try_from(value: String) -> Res<Self> {
        return InnerFile::try_from(InnerPath::try_from(value)?);
    }
}

// common file operators
#[async_trait]
pub trait Op {
    fn get(&self) -> &FileInfo;
    async fn parent(&self) -> Res<InnerFile>;
    async fn rename(&mut self, name: &str) -> Void;
    async fn delete(&self) -> Void;
    async fn open(&self) -> Void;
}

#[async_trait]
pub trait FileOp: Op {
    async fn view(&self) -> Void;
    async fn edit(&self) -> Void;
}

#[async_trait]
pub trait DirOp: Op {
    async fn list(&self) -> Res<Vec<InnerFile>>;
    async fn new_file(&self, name: &str) -> Void;
    async fn new_dir(&self, name: &str) -> Void;
    async fn goto(&self, child_path: &str) -> Res<InnerFile>;
    async fn shell(&self) -> Void;
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::model::file::local::make;
    use crate::model::file::{FileInfo, InnerFile};
    use crate::model::*;

    #[test]
    fn test_make() {
        file_info("/etc", |_is_dir, fi| {
            // assert_eq!(is_dir, true);
            assert_eq!(fi.path, "/etc");
            assert_eq!(fi.name, "etc");
            assert_eq!(fi.is_dir, true);
            assert_eq!(fi.mode, "drwxr-xr-x")
        });
    }

    fn file_info<F>(path: &str, ff: F)
    where
        F: FnOnce(bool, &FileInfo) -> (),
    {
        let ft = make(Path::new(path)).unwrap();
        match ft {
            InnerFile::File(file) => ff(false, file.as_ref().get()),
            InnerFile::Dir(dir) => ff(true, dir.as_ref().get()),
        }
    }
}
