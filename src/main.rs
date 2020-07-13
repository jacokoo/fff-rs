use std::env::current_dir;
use std::fs::{metadata, symlink_metadata, read_link};
use std::io::Result;

use tokio::fs::read_dir;
use tokio::stream::*;

use model::file::*;
use model::make;
use std::path::Path;
use std::os::macos::fs::MetadataExt;

mod ui;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let wd = current_dir()?;

    if let FileType::Dir(dir) = make(&wd)? {
        for item in dir.as_ref().list().await? {
            let info = item.info();
            println!("{}, {}, {}", info.name, info.path, info.mode);
            if let Some(link) = &info.link {
                println!("is link {}, {}", link.target, link.broken)
            }
        }
    }

    let pp = Path::new("/Users/guyong/ws/rust/fff/target/debug/fff.dSYM");
    let meta = metadata(pp)?;
    println!("{:o}, {}, {}", meta.st_mode(), meta.file_type().is_symlink(), meta.is_dir());

    let sm = symlink_metadata(pp)?;
    println!("{:o}, {}, {}", sm.st_mode(), sm.file_type().is_symlink(), sm.is_dir());

    let sp = read_link(pp)?;
    println!("{}", sp.display());
    Ok(())
}