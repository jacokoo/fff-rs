use std::path::PathBuf;

use async_trait::async_trait;
use std::fs;

use crate::model::file::local::file;
use crate::model::file::*;
use crate::model::result::{Error, Res, Void};
use std::fs::read_dir;

pub struct LocalDir(FileInfo);

#[async_trait]
impl Op for LocalDir {
    fn get(&self) -> &FileInfo {
        &self.0
    }
    async fn parent(&self) -> Res<InnerFile> {
        file::parent(&self.0)
    }
    async fn rename(&mut self, name: &str) -> Void {
        file::rename(&self.0, name)
    }
    async fn delete(&self) -> Void {
        file::delete(&self.0)
    }
    async fn open(&self) -> Void {
        file::open(&self.0)
    }
}

impl LocalDir {
    pub fn new(fi: FileInfo) -> LocalDir {
        LocalDir(fi)
    }
    fn join_path(&self, name: &str) -> PathBuf {
        self.0.get_path().join(name)
    }
}

#[async_trait]
impl DirOp for LocalDir {
    async fn list(&self) -> Res<Vec<InnerFile>> {
        let dir = read_dir(self.0.get_path())?;
        Ok(dir
            .filter(|d| {
                if let Ok(dd) = d {
                    let name = dd.file_name();
                    return name != "." && name != "..";
                }
                false
            })
            .map(|d| make(&d?.path()))
            .filter(|d| d.is_ok())
            .map(|d| d.unwrap())
            .collect::<Vec<InnerFile>>())
    }

    async fn new_file(&self, name: &str) -> Void {
        let p = self.join_path(name);
        if p.exists() {
            return Err(Error::FileAlreadyExists(p.display().to_string()));
        }
        fs::File::create(p)?;
        Ok(())
    }

    async fn new_dir(&self, name: &str) -> Void {
        fs::create_dir_all(self.join_path(name))?;
        Ok(())
    }

    async fn goto(&self, child_path: &str) -> Res<InnerFile> {
        make(&self.join_path(child_path))
    }

    async fn shell(&self) -> Void {
        Ok(())
    }
}
