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
                stdout().flush().unwrap();
            }
            EventBody::Batch(data, tx) => {
                ui.stop_loading();
                data.into_iter().for_each(|it| handle_single(&mut ui, it));
                ack(tx);
                stdout().flush().unwrap();
            }
            EventBody::Queue(data, tx) => {
                ui.start_loading();
                match data {
                    UIEvent::EndQueue => {
                        ui.stop_loading();
                        stdout().flush().unwrap();
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
        UIEvent::SwitchTab(idx) => ui.switch_tab(idx),
        UIEvent::StartLoading => ui.start_loading(),
        _ => println!("event: {:?}", ev),
    }
}

fn ack(tx: Option<Sender<bool>>) {
    if let Some(t) = tx {
        t.send(true).unwrap();
    }
}
