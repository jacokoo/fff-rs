use crate::model::file::path::InnerPath;
use crate::model::result::Void;
use crate::model::state::bookmark::Bookmark;
use crate::model::state::group::Group;


use crate::ui::event::{UIEventSender};
use std::convert::TryFrom;
use std::path::PathBuf;

pub enum ViewMode {
    InColumn,
    InList,
}

const MAX_GROUP_COUNT: usize = 4;

pub struct Workspace {
    enter_path: InnerPath,
    home_path: InnerPath,
    current_group: usize,
    current_mode: ViewMode,
    show_detail: bool,
    groups: Vec<Group>,
    ui_event: UIEventSender,
    bookmark: Bookmark,
}

impl Workspace {
    pub fn new(enter_path: &PathBuf, home_path: &PathBuf, ui_event: UIEventSender) -> Self {
        let bookmark = Bookmark::new(home_path);
        Workspace {
            enter_path: InnerPath::try_from(enter_path.display().to_string()).unwrap(),
            home_path: InnerPath::try_from(home_path.display().to_string()).unwrap(),
            current_group: 0,
            current_mode: ViewMode::InColumn,
            show_detail: false,
            groups: Vec::new(),
            ui_event,
            bookmark,
        }
    }

    pub async fn init_groups(&mut self) -> Void {
        self.ui_event.start_loading()?;
        for _ in 0..MAX_GROUP_COUNT {
            let mut g = Group::new();
            g.current_mut().update(&self.enter_path).await?;
            self.groups.push(g)
        }
        Ok(())
    }

    pub async fn switch_to(&mut self, tab: usize) -> Void {
        let t = if tab > MAX_GROUP_COUNT {
            MAX_GROUP_COUNT - 1
        } else {
            tab
        };

        let current = &self.groups[t];
        current.sync_to_ui(&self.ui_event)?;
        self.ui_event.end_queue()?;
        Ok(())
    }
}
