use crate::kbd::{Answer, Kbd};
use std::sync::Arc;

pub struct Context {
    kbd: Arc<Kbd>,
}

impl Context {
    pub fn new(kbd: Arc<Kbd>) -> Self {
        Context { kbd }
    }

    pub async fn request_input(&self, msg: &str) -> Option<String> {
        self.kbd.request_input(msg).await
    }

    pub async fn request_answer(&self, msg: &str, multiple: bool) -> Option<Answer> {
        self.kbd.request_answer(msg, multiple).await
    }

    pub fn message(&self, msg: &str) {}
}
