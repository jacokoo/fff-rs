use std::fs::{read_link, Metadata};
use std::os::macos::fs::MetadataExt;
use std::path::{Path};

use crate::model::file::file_mode::mode_string;
use crate::model::file::local::dir::LocalDir;
use crate::model::file::local::file::LocalFile;
use crate::model::file::path::InnerPath;
use crate::model::file::{FileInfo, InnerFile, LinkInfo};
use crate::model::result::{option_from_result, Error, Res};

mod dir;
mod file;

pub fn make(inner: InnerPath) -> Res<InnerFile> {
    let p = &inner.path;
    if !p.exists() {
        return Error::PathNotExists(inner.to_string()).res();
    }
    let meta = p.symlink_metadata()?;
    let v = make_it(&meta, inner);
    Ok(if meta.is_dir() {
        InnerFile::Dir(Box::new(LocalDir::new(v)))
    } else {
        InnerFile::File(Box::new(LocalFile::new(v)))
    })
}

fn make_it(meta: &Metadata, inner: InnerPath) -> FileInfo {
    let path = &inner.path;
    let name = path
        .file_name()
        .map(|r| r.to_str().unwrap())
        .unwrap_or("-")
        .to_string();
    let mode = mode_string(meta.st_mode());
    let link = if meta.file_type().is_symlink() {
        option_from_result(read_link(path)).map(|p| {
            let broken = !p.exists();
            let target = if p.is_relative() {
                path.parent().unwrap_or(Path::new("/")).join(p)
            } else {
                p
            };
            LinkInfo {
                broken,
                target: target.display().to_string(),
            }
        })
    } else {
        None
    };

    FileInfo {
        path: path.clone(),
        inner,
        name,
        size: meta.len(),
        mode,
        modified: option_from_result(meta.modified()),
        is_dir: meta.is_dir(),
        link,
        protocol: None,
    }
}
