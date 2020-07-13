mod file_mode;
mod local;
mod protocol;
pub mod file;

pub use local::make;

#[cfg(test)]
mod test {
    use crate::model::*;
    use crate::model::file::{FileType, FileInfo};
    use std::path::Path;

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

    fn file_info<F>(path: &str, ff: F) where F: FnOnce(bool, &FileInfo) -> () {
        let ft = make(Path::new(path)).unwrap();
        match ft {
            FileType::File(file) => ff(false, file.as_ref().get()),
            FileType::Dir(dir) => ff(true, dir.as_ref().get())
        }
    }
}