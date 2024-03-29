use crate::ui::event::UIEvent::*;
use crate::ui::event::{EventBody, UIEvent};
use crate::ui::main::ui::UI;
use crossbeam_channel::{Receiver, Sender};

pub fn handle(mut ui: UI, rx: Receiver<EventBody>) {
    let mut in_queue = false;
    while let Ok(ev) = rx.recv() {
        log::debug!("handle {:?}  {}", ev, in_queue);

        match ev {
            EventBody::Single(data, tx) => match data {
                UIEvent::StartQueue => {
                    in_queue = true;
                }
                UIEvent::EndQueue => {
                    in_queue = false;
                    ui.flush();
                }
                d => {
                    handle_single(&mut ui, d);
                    after_handle(&mut ui, tx, &in_queue);
                }
            },
            EventBody::Batch(data, tx) => {
                data.into_iter().for_each(|it| handle_single(&mut ui, it));
                after_handle(&mut ui, tx, &in_queue)
            }
        }
    }
}

fn handle_single(ui: &mut UI, ev: UIEvent) {
    match ev {
        SwitchTab(idx) => ui.switch_tab(idx),
        StartLoading => ui.start_loading(),
        SetBookmark(bs) => ui.board_mut().set_bookmark(bs),
        SetPath(p) => ui.path_mut().set_path(&p),
        InitColumn(fs) => ui.board_mut().init_files(fs),
        InitSelect(ss) => ui.board_mut().init_selected(ss),
        InitMark(ms) => ui.board_mut().init_marked(ms),
        ShowKeyNav(ns) => ui.show_key_nav(ns),
        RefreshFileItem(fs) => ui.board_mut().refresh_files(fs),
        SetMark(m) => ui.board_mut().set_mark(m),
        SetSelect(ss) => ui.board_mut().set_selected(ss),
        AddFileList(fs) => ui.board_mut().add_files(fs),
        RemoveFileList(fs) => ui.board_mut().remove_files(fs),
        SetShowDetail(show) => ui.board_mut().set_show_detail(show),
        InputEnter(p) => ui.show_input(p),
        InputUpdate(p, u) => ui.update_input(p, u),
        InputMove(u) => ui.update_input_cursor(u),
        InputQuit => ui.clear_message(),
        a => log::debug!("unhandled event: {:?}", a),
    }
}

fn after_handle(ui: &mut UI, tx: Option<Sender<bool>>, in_queue: &bool) {
    ui.stop_loading();
    ui.clear_message();

    if let Some(t) = tx {
        t.send(true).unwrap();
    }

    if !in_queue {
        ui.flush();
    }
}
