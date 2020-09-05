use crate::kbd::ActionReceiver;
use crate::model::state::list::{FileSortBy, FilterTrait, SelectorTrait, SorterTrait};
use crate::model::state::workspace::Workspace;

mod context;

pub async fn init_action(ac: ActionReceiver, mut ws: Workspace) {
    tokio::spawn(async move {
        while let Ok(s) = ac.0.recv() {
            match s.as_ref() {
                "ActionSortByName" => ws.current_list_mut().set_order(FileSortBy::NAME),
                "ActionSortByMtime" => ws.current_list_mut().set_order(FileSortBy::MTIME),
                "ActionSortBySize" => ws.current_list_mut().set_order(FileSortBy::SIZE),
                "ActionChangeGroup0" => ws.switch_to(0).await.unwrap(),
                "ActionChangeGroup1" => ws.switch_to(1).await.unwrap(),
                "ActionChangeGroup2" => ws.switch_to(2).await.unwrap(),
                "ActionChangeGroup3" => ws.switch_to(3).await.unwrap(),
                "ActionToggleHidden" => ws.current_list_mut().toggle_show_hidden(),
                "ActionToggleDetail" => ws.toggle_show_detail(),
                "ActionMoveUp" => {
                    ws.current_list_mut().move_select(-1);
                }
                "ActionMoveDown" => {
                    ws.current_list_mut().move_select(1);
                }
                "ActionMoveToFirst" => {
                    ws.current_list_mut().select_first();
                }
                "ActionMoveToLast" => {
                    ws.current_list_mut().select_last();
                }
                a => log::debug!("unhandled action {}", a),
            };
        }
    })
    .await
    .unwrap();
}
