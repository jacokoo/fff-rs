use crate::model::result::Error;
use fff_macros::*;
use std::convert::TryFrom;
use std::hash::Hash;

macro_rules! create_enum {
    ($name:ident: $($item:ident),*) => {
        #[derive(PartialEq, Eq, Hash, Debug)]
        pub enum $name {
            $($item, )*
        }

        impl TryFrom<&str> for $name {
            type Error = Error;

            fn try_from(s: &str) -> Result<$name, Error> {
                $(
                    if (kebab_str!($item) == s) { return Ok($name::$item); }
                )*
                Err(Error::InvalidEnumValue(format!("{} can not convert to enum {}", s, stringify!($name))))
            }
        }
    }
}

create_enum!(BindingType: All, Normal, Jump, Input, Task, Clip);
create_enum!(
    ColorType: Normal,
    Keyword,
    Folder,
    File,
    Marked,
    Statusbar,
    StatusbarTitle,
    Tab,
    Jump,
    Filter,
    Clip
);
