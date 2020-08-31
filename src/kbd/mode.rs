use crate::config::Bindings;
use std::collections::HashMap;

static S: u16 = 1;

pub struct Mode<T: Sized> {
    name: String,
    kbd: Bindings,
    subs: Option<HashMap<String, String>>,
    data: T,
}
