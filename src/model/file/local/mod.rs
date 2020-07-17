use std::fs::{read_link, Metadata};
use std::os::macos::fs::MetadataExt;
use std::path::Path;

use tokio::io::{Error, ErrorKind, Result};

use crate::model::file::file_mode::mode_string;
use crate::model::file::local::dir::LocalDir;
use crate::model::file::local::file::LocalFile;
use crate::model::file::{option_from_result, FileInfo, FileType, LinkInfo};

mod dir;
mod file;

pub fn make(p: &Path) -> Result<FileType> {
    if !p.exists() {
        return Result::Err(Error::from(ErrorKind::NotFound));
    }
    let meta = p.symlink_metadata()?;
    let v = make_it(p, &meta);
    Ok(if meta.is_dir() {
        FileType::Dir(Box::new(LocalDir::new(v)))
    } else {
        FileType::File(Box::new(LocalFile::new(v)))
    })
}

fn make_it(path: &Path, meta: &Metadata) -> FileInfo {
    let name = path
        .file_name()
        .map(|r| r.to_str().unwrap())
        .unwrap_or("")
        .to_string();
    let pp = path.display().to_string();
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
        name,
        path: pp,
        size: meta.len(),
        mode,
        modified: option_from_result(meta.modified()),
        is_dir: meta.is_dir(),
        link,
        protocol: None,
    }
}
