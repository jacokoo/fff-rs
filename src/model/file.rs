use std::io::Result;
use std::path::Path;
use std::time::SystemTime;

use async_trait::async_trait;

pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub mode: String,
    pub modified: Option<SystemTime>,
    pub is_dir: bool,
    pub link: Option<LinkInfo>,
    pub protocol: Option<ProtocolInfo>
}

impl FileInfo {
    pub fn get_path(&self) -> &Path { Path::new(&self.path) }
}

pub struct LinkInfo {
    pub broken: bool,
    pub target: String
}

pub struct ProtocolInfo {
    pub protocol: String,
    pub instance_id: u8,
    pub root: Box<FileInfo>
}

pub enum FileType {
    File(Box<dyn FileOp>),
    Dir(Box<dyn DirOp>),
}

impl FileType {
    pub fn info(&self) -> &FileInfo {
        match self {
            FileType::File(file) => file.get(),
            FileType::Dir(dir) => dir.get()
        }
    }
}

pub type Void = Result<()>;

// common file operators
#[async_trait]
pub trait Op {
    fn get(&self) -> &FileInfo;
    async fn parent(&self) -> Result<FileType>;
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
    async fn list(&self) -> Result<Vec<FileType>>;
    async fn new_file(&self, name: &str) -> Void;
    async fn new_dir(&self, name: &str) -> Void;
    async fn goto(&self, child_path: &str) -> Result<FileType>;
    async fn shell(&self) -> Void;
}

pub fn option_from_result<T, E>(r: std::result::Result<T, E>) -> Option<T> {
    if let Ok(t) = r { Some(t) } else { None }
}
