use crate::ui::event::{EventBody, UIEvent};
use crate::ui::main::ui::UI;
use crossbeam_channel::{Receiver, Sender};
use std::io::{stdout, Write};

pub fn handle(mut ui: UI, mut rx: Receiver<EventBody>) {
    while let Ok(ev) = rx.recv() {
        match ev {
            EventBody::Single(data, tx) => {
                handle_single(&mut ui, data);
                ack(tx);
            }
            EventBody::Batch(data, tx) => {
                data.into_iter().for_each(|it| handle_single(&mut ui, it));
                ack(tx);
            }
        }
        stdout().flush().unwrap();
    }
}

fn handle_single(ui: &mut UI, ev: UIEvent) {
    match ev {
        UIEvent::SwitchTab(idx) => ui.switch_tab(idx),
        UIEvent::Loading(loading) => ui.set_loading(loading),
        _ => println!("event: {:?}", ev),
    }
}

fn ack(tx: Option<Sender<bool>>) {
    if let Some(t) = tx {
        t.send(true).unwrap();
    }
}
