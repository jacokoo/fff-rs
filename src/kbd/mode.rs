use crate::model::config::Bindings;
use std::collections::HashMap;

pub struct Mode<T: Sized> {
    name: String,
    kbd: Bindings,
    subs: Option<HashMap<String, String>>,
    data: T,
}
