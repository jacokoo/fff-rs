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
    wd: PathBuf,
    home: PathBuf,
) -> Void {
    Ok(())
}
