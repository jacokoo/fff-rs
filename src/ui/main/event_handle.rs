use crate::ui::event::UIEvent::*;
use crate::ui::event::{EventBody, UIEvent};
use crate::ui::main::ui::UI;
use crossbeam_channel::{Receiver, Sender};

pub fn handle(mut ui: UI, rx: Receiver<EventBody>) {
    let mut in_queue = false;
    while let Ok(ev) = rx.recv() {
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
                    ui.stop_loading();
                    handle_single(&mut ui, d);
                    ack(tx);
                    if !in_queue {
                        ui.flush();
                    }
                }
            },
            EventBody::Batch(data, tx) => {
                ui.stop_loading();
                data.into_iter().for_each(|it| handle_single(&mut ui, it));
                ack(tx);
                if !in_queue {
                    ui.flush();
                }
            }
        }
    }
}

fn handle_single(ui: &mut UI, ev: UIEvent) {
    log::debug!("handle {:?}", ev);

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
        a => log::debug!("unhandled event: {:?}", a),
    }
}

fn ack(tx: Option<Sender<bool>>) {
    if let Some(t) = tx {
        t.send(true).unwrap();
    }
}
