use crate::kbd::ActionReceiver;
use crate::model::state::workspace::Workspace;

mod context;

pub async fn init_action(ac: ActionReceiver, mut wss: Workspace) {
    tokio::spawn(async move {
        let ws = &mut wss;
        while let Ok(s) = ac.0.recv() {
            match s.as_ref() {
                "ActionChangeGroup0" => ws.switch_to(0).await.unwrap(),
                "ActionChangeGroup1" => ws.switch_to(1).await.unwrap(),
                "ActionChangeGroup2" => ws.switch_to(2).await.unwrap(),
                "ActionChangeGroup3" => ws.switch_to(3).await.unwrap(),
                "ActionMoveUp" => ws.select(-1),
                a => log::debug!("unhandled action {}", a),
            };
        }
    })
    .await
    .unwrap();
}
