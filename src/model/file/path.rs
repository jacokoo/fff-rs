use crate::model::file::FileInfo;
use crate::model::result::Error;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PathProtocolInfo {
    pub protocol: String,
    pub path: PathBuf,
}

// /etc/ssh/hello.ssh.fff@ssh:///home/foo/bar
// ^----- path ---------^^- p -^^- p/path --^
#[derive(Debug, Clone)]
pub struct InnerPath {
    origin: String,
    pub path: PathBuf,
    pub protocol: Option<PathProtocolInfo>,
}

impl InnerPath {
    fn from_path(value: PathBuf) -> Result<Self, Error> {
        if !value.exists() {
            return Err(Error::PathNotExists(value.display().to_string()));
        }

        return Ok(InnerPath {
            origin: value.display().to_string(),
            path: value,
            protocol: None,
        });
    }
}

impl TryFrom<&PathBuf> for InnerPath {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        InnerPath::from_path(value.clone())
    }
}

impl TryFrom<PathBuf> for InnerPath {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        InnerPath::from_path(value)
    }
}

impl TryFrom<&Path> for InnerPath {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        InnerPath::from_path(value.into())
    }
}

impl TryFrom<String> for InnerPath {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(idx) = value.find("@") {
            let first = value[..idx].to_string();
            let second = value[idx + 1..].to_string();

            if !Path::new(&first).exists() {
                return Err(Error::PathNotExists(first.to_string()));
            }

            let ptk: Vec<_> = second.split("://").collect();
            if ptk.len() != 2 {
                return Err(Error::InvalidProtocolString(second.to_string()));
            }

            return Ok(InnerPath {
                origin: value,
                path: first.into(),
                protocol: Some(PathProtocolInfo {
                    protocol: ptk[0].to_owned(),
                    path: ptk[1].into(),
                }),
            });
        }

        if !Path::new(&value).exists() {
            return Err(Error::PathNotExists(value));
        }

        return Ok(InnerPath {
            path: (&value).into(),
            protocol: None,
            origin: value,
        });
    }
}

impl ToString for InnerPath {
    fn to_string(&self) -> String {
        self.origin.clone()
    }
}
