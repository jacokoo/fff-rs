use crate::kbd::ActionReceiver;
use crate::model::result::Res;
use crate::model::state::list::{FileSortBy, FilterTrait, SelectorTrait, SorterTrait};
use crate::model::state::workspace::Workspace;
use crate::ui::event::UIEventSender;

mod context;

fn ok<T>(_: T) -> Res<()> {
    Ok(())
}

pub async fn init_action(ac: ActionReceiver, mut ws: Workspace, sender: UIEventSender) {
    tokio::spawn(async move {
        while let Ok(s) = ac.0.recv() {
            sender.start_queue();
            let res = match s.as_ref() {
                "ActionSortByName" => ok(ws.current_list_mut().set_order(FileSortBy::NAME)),
                "ActionSortByMtime" => ok(ws.current_list_mut().set_order(FileSortBy::MTIME)),
                "ActionSortBySize" => ok(ws.current_list_mut().set_order(FileSortBy::SIZE)),
                "ActionMoveUp" => ok(ws.current_list_mut().move_select(-1)),
                "ActionMoveDown" => ok(ws.current_list_mut().move_select(1)),
                "ActionOpenFolderRight" => ws.open_selected().await,
                "ActionCloseFolderRight" => ws.close_right().await,
                "ActionChangeGroup0" => ws.switch_to(0).await,
                "ActionChangeGroup1" => ws.switch_to(1).await,
                "ActionChangeGroup2" => ws.switch_to(2).await,
                "ActionChangeGroup3" => ws.switch_to(3).await,
                "ActionToggleHidden" => ok(ws.current_list_mut().toggle_show_hidden()),
                "ActionToggleDetail" => ok(ws.toggle_show_detail()),
                "ActionMoveToFirst" => ok(ws.current_list_mut().select_first()),
                "ActionMoveToLast" => ok(ws.current_list_mut().select_last()),
                a => ok(log::debug!("unhandled action {}", a)),
            };
            sender.end_queue();

            if let Err(e) = res {
                log::error!("error {:?}", e);
            }
        }
    })
    .await
    .unwrap();
}
