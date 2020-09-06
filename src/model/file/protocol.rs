use crate::model::file::InnerFile;
use async_trait::async_trait;
use std::io::Result;

#[async_trait]
pub trait Protocol {
    fn name(&self) -> &'static str;
    fn support(&self, file: &InnerFile) -> bool;
    async fn create(&mut self, file: &InnerFile) -> Result<&InnerFile>;
}
