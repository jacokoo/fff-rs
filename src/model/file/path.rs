use crate::model::file::FileInfo;
use crate::model::result::Error;
use std::convert::TryFrom;
use std::path::Path;

pub struct PathProtocolInfo {
    pub protocol: String,
    pub path: String,
}

// /etc/ssh/hello.ssh.fff@ssh:///home/foo/bar
// ^----- path ---------^^- p -^^- p/path --^
pub struct InnerPath {
    pub path: String,
    pub protocol: Option<PathProtocolInfo>,
}

impl TryFrom<String> for InnerPath {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(_idx) = value.find("@") {
            let tk: Vec<_> = value.split("@").collect();
            if tk.len() != 2 {
                return Err(Error::InvalidPath(value));
            }

            if !Path::new(tk[0]).exists() {
                return Err(Error::PathNotExists(tk[0].to_owned()));
            }

            let p = tk[1].to_owned();
            let ptk: Vec<_> = p.split("://").collect();
            if ptk.len() != 2 {
                return Err(Error::InvalidProtocolString(p));
            }

            return Ok(InnerPath {
                path: tk[0].to_owned(),
                protocol: Some(PathProtocolInfo {
                    protocol: ptk[0].to_owned(),
                    path: ptk[1].to_owned(),
                }),
            });
        }

        if !Path::new(&value).exists() {
            return Err(Error::PathNotExists(value));
        }

        return Ok(InnerPath {
            path: value,
            protocol: None,
        });
    }
}

impl From<&FileInfo> for InnerPath {
    fn from(fi: &FileInfo) -> Self {
        if let Some(p) = &fi.protocol {
            return InnerPath {
                path: p.root.path.clone(),
                protocol: Some(PathProtocolInfo {
                    protocol: p.protocol.clone(),
                    path: fi.path.clone(),
                }),
            };
        }

        return InnerPath {
            path: fi.path.clone(),
            protocol: None,
        };
    }
}

impl ToString for InnerPath {
    fn to_string(&self) -> String {
        self.path.clone()
    }
}
