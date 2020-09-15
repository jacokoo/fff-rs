use crate::kbd::action::*;
use crate::kbd::Kbd;
use crate::kbd::ActionReceiver;
use crate::model::result::Res;
use crate::model::state::list::{FileSortBy, MarkerTrait, SelectorTrait};
use crate::model::state::workspace::Workspace;
use crate::ui::event::{UIEvent, UIEventSender};
use std::sync::Arc;

fn ok<T>(_: T) -> Res<()> {
    Ok(())
}

pub async fn init_action(ac: ActionReceiver, mut ws: Workspace, sender: UIEventSender, kbd: Arc<Kbd>) {
    tokio::spawn(async move {
        while let Ok(s) = ac.0.recv() {
            sender.start_queue().unwrap();
            let res = match s.as_ref() {
                NORMAL_SORT_BY_NAME => ok(ws.set_order(FileSortBy::NAME)),
                NORMAL_SORT_BY_MTIME => ok(ws.set_order(FileSortBy::MTIME)),
                NORMAL_SORT_BY_SIZE => ok(ws.set_order(FileSortBy::SIZE)),
                NORMAL_MOVE_UP => ok(ws.current_list_mut().move_select(-1)),
                NORMAL_MOVE_DOWN => ok(ws.current_list_mut().move_select(1)),
                NORMAL_OPEN_FOLDER => ws.open_selected().await,
                NORMAL_CLOSE_FOLDER => ws.close_right().await,
                NORMAL_GROUP_0 => ws.switch_to(0).await,
                NORMAL_GROUP_1 => ws.switch_to(1).await,
                NORMAL_GROUP_2 => ws.switch_to(2).await,
                NORMAL_GROUP_3 => ws.switch_to(3).await,
                NORMAL_TOGGLE_HIDDEN => ok(ws.toggle_show_hidden()),
                NORMAL_TOGGLE_DETAIL => ok(ws.toggle_show_detail()),
                NORMAL_MOVE_FIRST => ok(ws.current_list_mut().select_first()),
                NORMAL_MOVE_LAST => ok(ws.current_list_mut().select_last()),
                NORMAL_TOGGLE_MARK => ok(ws.toggle_mark()),
                NORMAL_TOGGLE_MARK_ALL => ok(ws.current_list_mut().toggle_mark_all()),
                NORMAL_NEW_FILE => ws.new_file().await,
                INPUT_QUIT_ACTION => {
                    kbd.switch_to_normal();
                    ok(sender.send(UIEvent::InputQuit).unwrap())
                }
                "Quit" => break,
                a => ok(log::debug!("unhandled action {}", a)),
            };
            sender.end_queue().unwrap();

            if let Err(e) = res {
                log::error!("error {:?}", e);
            }
        }
    })
    .await
    .unwrap();

    log::debug!("action thread ended");
}
