use crate::model::file::path::InnerPath;
use crate::model::result::Void;
use crate::model::state::bookmark::Bookmark;
use crate::model::state::group::Group;
use crate::model::state::list::list::FileList;
use crate::model::state::list::SelectorTrait;
use crate::ui::event::UIEvent::{
    Message, RefreshFileItem, SetBookmark, SetMark, SetSelect, SetShowDetail, SwitchTab,
};
use crate::ui::event::{FileItem, UIEventSender};
use std::borrow::Borrow;
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
    pub fn new(enter_path: PathBuf, home_path: PathBuf, ui_event: UIEventSender) -> Self {
        let bookmark = Bookmark::new(&home_path);
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

    pub async fn init(&mut self) -> Void {
        for _ in 0..MAX_GROUP_COUNT {
            let mut g = Group::new();
            g.current_mut().update(self.enter_path.clone()).await?;
            self.bind_list(g.current_mut());
            self.groups.push(g)
        }
        self.ui_event
            .queue(SetBookmark(self.bookmark.keys().clone()))?;
        Ok(())
    }

    pub async fn switch_to(&mut self, tab: usize) -> Void {
        let t = if tab > MAX_GROUP_COUNT {
            MAX_GROUP_COUNT - 1
        } else {
            tab
        };

        self.current_group = t;
        let current = &self.groups[t];
        self.ui_event.queue(SwitchTab(t))?;
        current.sync_to_ui(&self.ui_event)?;
        self.ui_event.end_queue()?;
        Ok(())
    }

    pub fn current(&self) -> &Group {
        &self.groups[self.current_group]
    }

    pub fn current_mut(&mut self) -> &mut Group {
        &mut self.groups[self.current_group]
    }

    pub fn current_list(&self) -> &FileList {
        self.current().current()
    }

    pub fn current_list_mut(&mut self) -> &mut FileList {
        self.current_mut().current_mut()
    }

    pub fn toggle_show_detail(&mut self) {
        self.show_detail = !self.show_detail;
        self.ui_event.send(SetShowDetail(self.show_detail)).unwrap();
    }

    pub async fn open_selected(&mut self) {
        let of = self.current_list_mut().selected_file();
        match of {
            Some(file) => {
                if file.is_dir() {
                    self.current_mut().add_file_list().await;
                } else {
                    self.ui_event
                        .send(Message(format!("Can not open {}", file.path_str())))
                        .unwrap();
                }
            }
            None => {
                self.ui_event
                    .send(Message("No dir is selected".to_string()))
                    .unwrap();
            }
        }
    }

    fn bind_list(&self, list: &mut FileList) {
        let s1 = self.ui_event.clone();
        list.subscribe_file_change(move |fs| {
            s1.send(RefreshFileItem(
                fs.iter().map(|f| FileItem::from(f.borrow())).collect(),
            ))
            .unwrap();
        });

        let s2 = self.ui_event.clone();
        list.subscribe_mark_change(move |m| {
            s2.send(SetMark(m.iter().map(|it| it.clone()).collect()))
                .unwrap();
        });

        let s3 = self.ui_event.clone();
        list.subscribe_select_change(move |s| {
            s3.send(SetSelect(Some(s.clone()))).unwrap();
        });
    }
}
