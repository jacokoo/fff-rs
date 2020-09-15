use crate::model::context::Context;
use crate::model::file::{InnerFile, Op};
use crate::model::result::Res;
use crate::model::state::list::list::FileList;
use crate::model::state::list::{MarkerTrait, SelectorTrait};
use crate::model::state::workspace::ViewMode;
use crate::ui::event::UIEvent::*;
use crate::ui::event::{FileItem, UIEventResult, UIEventSender};
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
        ctx: &Context,
    ) -> Res<&mut FileList> {
        if let ViewMode::InColumn = mode {
            self.file_list.push(FileList::new());
        }

        self.current_mut().update_dir(file, ctx).await?;
        Ok(self.current_mut())
    }

    pub async fn close_last(&mut self, ctx: &Context) -> Res<(bool, Option<Vec<FileItem>>)> {
        if self.file_list.len() == 1 {
            return match self.current_mut().dir() {
                Some(v) => {
                    let p = v.parent(ctx).await?;
                    self.current_mut().update_dir(Arc::new(p), ctx).await?;
                    Ok((true, Some(self.current().file_items())))
                }
                None => Ok((false, None)),
            };
        }

        self.file_list.pop();
        Ok((true, None))
    }

    pub fn current(&self) -> &FileList {
        &self.file_list.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut FileList {
        let idx = self.file_list.len() - 1;
        &mut self.file_list[idx]
    }

    pub fn current_path(&self) -> String {
        self.current()
            .dir()
            .map_or("-".to_string(), |it| it.path_str())
    }

    pub fn sync_to_ui(&self, event: &UIEventSender) -> UIEventResult {
        event.batch_send(vec![
            SetPath(self.current_path()),
            InitColumn(self.map(|fl| fl.file_items())),
            InitSelect(self.map(|fl| fl.selected())),
            InitMark(self.map(|fl| fl.marked())),
        ])?;
        Ok(())
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&FileList) -> R,
    {
        self.file_list.iter().map(f).collect()
    }
}
