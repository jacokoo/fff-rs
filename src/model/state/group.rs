use crate::model::file::InnerFile;
use crate::model::result::Res;
use crate::model::state::list::list::FileList;
use crate::model::state::list::{MarkerTrait, SelectorTrait};
use crate::model::state::workspace::ViewMode;
use crate::ui::event::UIEvent::*;
use crate::ui::event::{UIEventResult, UIEventSender};
use std::sync::Arc;

pub struct Group {
    file_list: Vec<FileList>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            file_list: vec![FileList::new()],
        }
    }

    pub async fn add_file_list(
        &mut self,
        file: Arc<InnerFile>,
        mode: &ViewMode,
    ) -> Res<&mut FileList> {
        if let ViewMode::InColumn = mode {
            self.file_list.push(FileList::new());
        }

        self.current_mut().update_dir(file).await?;
        Ok(self.current_mut())
    }

    pub fn current(&self) -> &FileList {
        &self.file_list.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut FileList {
        let idx = self.file_list.len() - 1;
        &mut self.file_list[idx]
    }

    pub fn items(&self) -> &Vec<FileList> {
        &self.file_list
    }

    pub fn sync_to_ui(&self, event: &UIEventSender) -> UIEventResult {
        event.queue(SetPath(
            self.current()
                .dir()
                .as_ref()
                .map_or("-".to_string(), |it| it.path_str()),
        ))?;
        event.queue(InitColumn(self.map(|fl| fl.file_items())))?;
        event.queue(InitSelect(self.map(|fl| fl.selected())))?;
        event.queue(InitMark(self.map(|fl| fl.marked())))?;
        Ok(())
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&FileList) -> R,
    {
        self.file_list.iter().map(f).collect()
    }
}
