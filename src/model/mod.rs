use crate::kbd::ActionReceiver;
use crate::model::result::{Error, Void};
use crate::model::state::workspace::Workspace;
use crate::ui::event::UIEventSender;
use std::path::PathBuf;

pub mod file;
pub mod result;
pub mod state;

pub async fn init_state(
    ac: ActionReceiver,
    sender: UIEventSender,
    wd: &PathBuf,
    home: &PathBuf,
) -> Void {
    tokio::spawn(async move {
        let mut ws = Workspace::new(&wd, &home, sender);
        ws.init().await;
        ws.switch_to(0).await;

        while let Ok(s) = ac.0.recv() {
            match s.as_ref() {
                "ActionChangeGroup0" => ws.switch_to(0).await.unwrap(),
                "ActionChangeGroup1" => ws.switch_to(1).await.unwrap(),
                "ActionChangeGroup2" => ws.switch_to(2).await.unwrap(),
                "ActionChangeGroup3" => ws.switch_to(3).await.unwrap(),
                "ActionMoveUp" => ws.select(-1),
                a => log::debug!("unhandled action {}", a),
            }
        }
    })
    .await
    .map_err(|e| Error::JoinError(e))
}
