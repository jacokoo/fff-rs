use std::io::Result;

use async_trait::async_trait;

use crate::model::file::FileType;

#[async_trait]
trait Protocol {
    fn name(&self) -> &'static str;
    fn support(&self, file: &FileType) -> bool;
    async fn create(&mut self, file: &FileType) -> Result<&FileType>;
}