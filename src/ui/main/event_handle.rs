use crate::ui::event::UIEvent::*;
use crate::ui::event::{EventBody, UIEvent};
use crate::ui::main::ui::UI;
use crossbeam_channel::{Receiver, Sender};
use std::io::{stdout, Write};

pub fn handle(mut ui: UI, rx: Receiver<EventBody>) {
    while let Ok(ev) = rx.recv() {
        match ev {
            EventBody::Single(data, tx) => {
                ui.stop_loading();
                handle_single(&mut ui, data);
                ack(tx);
                ui.flush();
            }
            EventBody::Batch(data, tx) => {
                ui.stop_loading();
                data.into_iter().for_each(|it| handle_single(&mut ui, it));
                ack(tx);
                ui.flush();
            }
            EventBody::Queue(data, tx) => {
                ui.start_loading();
                match data {
                    EndQueue => {
                        ui.stop_loading();
                        ui.flush();
                        ui.flush();
                    }
                    a => {
                        handle_single(&mut ui, a);
                    }
                }
                ack(tx);
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
        a => log::debug!("{:?}", a),
    }
}

fn ack(tx: Option<Sender<bool>>) {
    if let Some(t) = tx {
        t.send(true).unwrap();
    }
}
