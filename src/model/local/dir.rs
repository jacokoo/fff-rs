use std::io::Result;
use std::path::PathBuf;

use async_trait::async_trait;
use std::io::ErrorKind;
use std::io::Error;
use std::fs;

use crate::model::file::*;
use crate::model::local::file::*;
use crate::model::make;
use std::fs::read_dir;

pub struct LocalDir(FileInfo);

#[async_trait]
impl Op for LocalDir {
    fn get(&self) -> &FileInfo { &self.0 }
    async fn parent(&self) -> Result<FileType> { parent(&self.0) }
    async fn rename(&mut self, name: &str) -> Void { rename(&self.0, name) }
    async fn delete(&self) -> Void { delete(&self.0) }
    async fn open(&self) -> Void { open(&self.0) }
}

impl LocalDir {
    pub fn new(fi: FileInfo) -> LocalDir { LocalDir(fi) }
    fn join_path(&self, name: &str) -> PathBuf { self.0.get_path().join(name) }
}

#[async_trait]
impl DirOp for LocalDir {
    async fn list(&self) -> Result<Vec<FileType>> {
        let dir = read_dir(self.0.get_path())?;
        Ok(dir
            .filter(|d| {
                if let Ok(dd) = d {
                    let name = dd.file_name();
                    return name != "." && name != "..";
                }
                false
            })
            .map(|d| { make(&d?.path()) })
            .filter(|d| { d.is_ok() })
            .map(|d| { d.unwrap() })
            .collect::<Vec<FileType>>()
        )
    }

    async fn new_file(&self, name: &str) -> Result<()> {
        let p = self.join_path(name);
        if p.exists() {
            return Err(Error::from(ErrorKind::AlreadyExists));
        }
        fs::File::create(p)?;
        Ok(())
    }

    async fn new_dir(&self, name: &str) -> Void {
        fs::create_dir_all(self.join_path(name))?;
        Ok(())
    }

    async fn goto(&self, child_path: &str) -> Result<FileType> {
        make(&self.join_path(child_path))
    }

    async fn shell(&self) -> Void {
        Ok(())
    }
}