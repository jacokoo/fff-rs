use crossbeam_channel::{bounded, Receiver, SendError, Sender};

#[derive(Debug)]
pub struct FileItem {
    pub name: String,
    pub detail: String,
    pub size: String,
    pub is_dir: bool,
}

#[derive(Debug)]
pub enum UIEvent {
    StartLoading,
    Message(String),

    SwitchTab(usize),
    SetPath(String),
    SetBookmark(Vec<String>),
    AddColumn(Vec<FileItem>),
    ShowKeyNav(Vec<(String, String)>),
}

#[derive(Debug)]
pub enum EventBody {
    Single(UIEvent, Option<Sender<bool>>),
    Batch(Vec<UIEvent>, Option<Sender<bool>>),
}

pub struct UIEventSender(Sender<EventBody>);

impl UIEventSender {
    pub fn new() -> (Self, Receiver<EventBody>) {
        let (tx, rx) = bounded(10);
        (UIEventSender(tx), rx)
    }

    pub fn start_loading(&mut self) -> Result<(), SendError<EventBody>> {
        self.send_async(UIEvent::StartLoading)
    }

    pub fn send_sync(&mut self, event: UIEvent) -> Result<(), SendError<EventBody>> {
        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Single(event, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn send_async(&mut self, event: UIEvent) -> Result<(), SendError<EventBody>> {
        self.0.send(EventBody::Single(event, None))
    }

    pub fn batch_send_sync(&mut self, events: Vec<UIEvent>) -> Result<(), SendError<EventBody>> {
        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Batch(events, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn batch_send_async(&mut self, events: Vec<UIEvent>) -> Result<(), SendError<EventBody>> {
        self.0.send(EventBody::Batch(events, None))
    }
}

impl Clone for UIEventSender {
    fn clone(&self) -> Self {
        UIEventSender(self.0.clone())
    }
}
