use std::path::Path;

use async_trait::async_trait;
use std::io::ErrorKind;

use crate::model::file::*;
use crate::model::result::{Error, Res, Void};

pub struct LocalFile(FileInfo);

impl LocalFile {
    pub fn new(fi: FileInfo) -> LocalFile {
        LocalFile(fi)
    }
}

pub fn get(info: &FileInfo) -> &FileInfo {
    info
}

pub fn parent(info: &FileInfo) -> Res<InnerFile> {
    match Path::new(&info.path).parent() {
        Some(p) => make(&p),
        None => Err(Error::Io(ErrorKind::NotFound)),
    }
}

pub fn rename(info: &FileInfo, name: &str) -> Void {
    let n = Path::new(&info.path).parent().map(move |p| p.join(name));

    if let Some(nn) = n {
        std::fs::rename(Path::new(&info.path), nn)?;
    }
    Ok(())
}

pub fn delete(_info: &FileInfo) -> Void {
    Ok(())
}

pub fn open(_info: &FileInfo) -> Void {
    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
    } else if cfg!(target_os = "macos") {
    }
    Ok(())
}

#[async_trait]
impl Op for LocalFile {
    fn get(&self) -> &FileInfo {
        &self.0
    }
    async fn parent(&self) -> Res<InnerFile> {
        parent(&self.0)
    }
    async fn rename(&mut self, name: &str) -> Void {
        rename(&self.0, name)
    }
    async fn delete(&self) -> Void {
        delete(&self.0)
    }
    async fn open(&self) -> Void {
        open(&self.0)
    }
}

#[async_trait]
impl FileOp for LocalFile {
    async fn view(&self) -> Void {
        Ok(())
    }
    async fn edit(&self) -> Void {
        Ok(())
    }
}
