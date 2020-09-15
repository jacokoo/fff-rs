use crate::model::context::Context;
use crate::model::file::local::file;
use crate::model::file::path::InnerPath;
use crate::model::file::*;
use crate::model::result::{Error, Res, Void};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::fs;
use std::fs::read_dir;
use std::path::PathBuf;

pub struct LocalDir(FileInfo);

#[async_trait]
impl Op for LocalDir {
    fn get(&self) -> &FileInfo {
        &self.0
    }
    async fn parent(&self, _: &Context) -> Res<InnerFile> {
        file::parent(&self.0)
    }
    async fn rename(&self, ctx: &Context) -> Void {
        file::rename(&self.0, ctx).await
    }
    async fn delete(&self, _: &Context) -> Void {
        file::delete(&self.0)
    }
    async fn open(&self, _: &Context) -> Void {
        file::open(&self.0)
    }
}

impl LocalDir {
    pub fn new(fi: FileInfo) -> LocalDir {
        LocalDir(fi)
    }
    fn join_path(&self, name: &str) -> PathBuf {
        self.0.inner.path.join(name)
    }
}

#[async_trait]
impl DirOp for LocalDir {
    async fn list(&self, _: &Context) -> Res<Vec<InnerFile>> {
        let dir = read_dir(&self.0.inner.path)?;
        Ok(dir
            .filter(|d| {
                if let Ok(dd) = d {
                    let name = dd.file_name();
                    return name != "." && name != "..";
                }
                false
            })
            .map(|d| make(InnerPath::try_from(&d?.path())?))
            .filter(|d| d.is_ok())
            .map(|d| d.unwrap())
            .collect::<Vec<InnerFile>>())
    }

    async fn new_file(&self, ctx: &Context) -> Void {
        if let Some(name) = ctx.request_input("New File").await {
            let p = self.join_path(&name);
            if p.exists() {
                ctx.message("File is already exists.");
                return Err(Error::FileAlreadyExists(p.display().to_string()));
            }
            fs::File::create(p)?;
        }
        Ok(())
    }

    async fn new_dir(&self, ctx: &Context) -> Void {
        if let Some(name) = ctx.request_input("New Dir").await {
            fs::create_dir_all(self.join_path(&name))?;
        }
        Ok(())
    }

    async fn goto(&self, _: &Context, child_path: &str) -> Res<InnerFile> {
        make(InnerPath::try_from(&self.join_path(child_path))?)
    }

    async fn shell(&self, _: &Context) -> Void {
        Ok(())
    }
}
