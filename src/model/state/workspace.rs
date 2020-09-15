use crate::common::Functional;
use crate::kbd::Kbd;
use crate::model::context::Context;
use crate::model::file::path::InnerPath;
use crate::model::result::Void;
use crate::model::state::bookmark::Bookmark;
use crate::model::state::group::Group;
use crate::model::state::list::list::FileList;
use crate::model::state::list::{FileSortBy, FilterTrait, MarkerTrait, SelectorTrait, SorterTrait};
use crate::ui::event::UIEvent::{
    AddFileList, Message, RefreshFileItem, RemoveFileList, SetBookmark, SetMark, SetPath,
    SetSelect, SetShowDetail, SwitchTab,
};
use crate::ui::event::{FileItem, UIEventSender};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::Arc;

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
    context: Arc<Context>,
    kbd: Arc<Kbd>,
}

impl Workspace {
    pub fn new(
        enter_path: PathBuf,
        home_path: PathBuf,
        ui_event: UIEventSender,
        kbd: Arc<Kbd>,
    ) -> Self {
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
            context: Arc::new(Context::new(kbd.clone())),
            kbd,
        }
    }

    pub async fn init(&mut self) -> Void {
        let ctx = self.context.borrow();
        for _ in 0..MAX_GROUP_COUNT {
            let mut g = Group::new();
            g.current_mut().update(self.enter_path.clone(), ctx).await?;
            Workspace::bind_list(&self.ui_event, g.current_mut());
            self.groups.push(g)
        }
        self.ui_event.start_queue().unwrap();
        self.ui_event
            .send(SetBookmark(self.bookmark.keys().clone()))?;
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
        self.ui_event.start_queue().unwrap();
        self.ui_event.send(SwitchTab(t))?;
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

    pub async fn close_right(&mut self) -> Void {
        let ctx = self.context.clone();
        let (succ, vs) = self.current_mut().close_last(ctx.borrow()).await?;
        if !succ {
            return Ok(());
        }

        let some = vs.is_some();
        self.ui_event.send(RemoveFileList(vs))?;
        self.ui_event.send(SetPath(self.current().current_path()))?;

        if some {
            self.ui_event
                .send(SetSelect(self.current_list().selected()))?;
        }
        Ok(())
    }

    pub fn set_order(&mut self, order: FileSortBy) {
        self.keep_select(move |s| {
            s.current_list_mut().set_order(order);
        });
    }

    pub fn toggle_show_hidden(&mut self) {
        self.keep_select(|s| {
            s.current_list_mut().toggle_show_hidden();
        });
    }

    pub fn toggle_mark(&mut self) {
        if let Some(idx) = self.current_list().selected() {
            self.current_list_mut().also(|it| {
                it.toggle_mark(idx);
                it.move_select(1);
            });
        }
    }

    fn keep_select<T: FnOnce(&mut Workspace)>(&mut self, f: T) {
        let sn = self.current_list().selected_file();
        {
            f(self);
        }
        if let Some(n) = sn {
            self.current_list_mut()
                .select_by_name(n.info().name.as_ref());
        }
    }

    pub async fn open_selected(&mut self) -> Void {
        let of = self.current_list_mut().selected_file();
        let ctx = self.context.clone();
        match of {
            Some(file) => {
                if file.is_dir() {
                    let mode = match self.current_mode {
                        ViewMode::InColumn => ViewMode::InColumn,
                        ViewMode::InList => ViewMode::InList,
                    };
                    self.current_list_mut().clear_mark();
                    let sender = self.ui_event.clone();
                    let fl = self
                        .current_mut()
                        .add_file_list(file.clone(), &mode, ctx.borrow())
                        .await?;
                    Workspace::bind_list(&sender, fl);
                    let vs = self.current_list().file_items();
                    self.ui_event.batch_send(vec![
                        SetPath(self.current().current_path()),
                        AddFileList(vs),
                        SetSelect(self.current_list().selected()),
                    ])?;
                } else {
                    self.ui_event
                        .send(Message(format!("Can not open {}", file.path_str())))?;
                }
            }
            None => {
                self.ui_event
                    .send(Message("No dir is selected".to_string()))?;
            }
        }
        Ok(())
    }

    fn bind_list(sender: &UIEventSender, list: &mut FileList) {
        let s1 = sender.clone();
        list.subscribe_file_change(move |fs| {
            s1.send(RefreshFileItem(
                fs.iter().map(|f| FileItem::from(f.borrow())).collect(),
            ))
            .unwrap();
        });

        let s2 = sender.clone();
        list.subscribe_mark_change(move |m| {
            s2.send(SetMark(m.iter().map(|it| it.clone()).collect()))
                .unwrap();
        });

        let s3 = sender.clone();
        list.subscribe_select_change(move |s| {
            s3.send(SetSelect(Some(s.clone()))).unwrap();
        });
    }
}
