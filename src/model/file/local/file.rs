use crate::model::context::Context;
use crate::model::file::path::InnerPath;
use crate::model::file::*;
use crate::model::result::{Error, Res, Void};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io::ErrorKind;

pub struct LocalFile(FileInfo);

impl LocalFile {
    pub fn new(fi: FileInfo) -> LocalFile {
        LocalFile(fi)
    }
}

pub fn parent(info: &FileInfo) -> Res<InnerFile> {
    match info.path.parent() {
        Some(p) => make(InnerPath::try_from(p)?),
        None => Err(Error::Io(ErrorKind::NotFound)),
    }
}

pub async fn rename(info: &FileInfo, ctx: &Context) -> Void {
    if let Some(name) = ctx.request_input("New file name").await {
        if let Some(nn) = &info.path.parent().map(move |p| p.join(name)) {
            if nn.exists() {
                ctx.message("The new file name is already exists, rename failed.");
            } else {
                std::fs::rename(&info.path, nn)?;
                ctx.message("Rename success.");
            }
        }
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
    async fn parent(&self, _: &Context) -> Res<InnerFile> {
        parent(&self.0)
    }
    async fn rename(&self, ctx: &Context) -> Void {
        rename(&self.0, ctx).await
    }
    async fn delete(&self, _: &Context) -> Void {
        delete(&self.0)
    }
    async fn open(&self, _: &Context) -> Void {
        open(&self.0)
    }
}

#[async_trait]
impl FileOp for LocalFile {
    async fn view(&self, _: &Context) -> Void {
        Ok(())
    }
    async fn edit(&self, _: &Context) -> Void {
        Ok(())
    }
}
