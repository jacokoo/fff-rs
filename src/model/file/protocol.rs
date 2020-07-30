use std::io::Result;

use async_trait::async_trait;

use crate::model::file::InnerFile;

#[async_trait]
pub trait Protocol {
    fn name(&self) -> &'static str;
    fn support(&self, file: &InnerFile) -> bool;
    async fn create(&mut self, file: &InnerFile) -> Result<&InnerFile>;
}
