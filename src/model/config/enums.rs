use fff_macros::*;
use std::convert::TryFrom;
use std::hash::Hash;
use termion::color::*;

macro_rules! create_enum {
    ($name:ident: $($item:ident),*) => {
        #[derive(PartialEq, Eq, Hash, Debug)]
        pub enum $name {
            $($item, )*
        }

        impl TryFrom<&str> for $name {
            type Error = String;

            fn try_from(s: &str) -> Result<$name, String> {
                $(
                    if (kebab_str!($item) == s) { return Ok($name::$item); }
                )*
                Err(format!("{} can not convert to enum {}", s, stringify!($name)))
            }
        }
    }
}

macro_rules! create_enum2 {
    ($name:ident: $($item:ident),*) => {
        #[derive(Debug)]
        pub enum $name {
            $($item($item), )*
        }

        impl From<&str> for $name {
            fn from(s: &str) -> $name {
                $(
                    if (kebab_str!($item) == s) { return $name::$item($item); }
                )*
                panic!("{} can not convert to enum {}", s, stringify!($name))
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
create_enum2!(
    ColorValue: Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
);
