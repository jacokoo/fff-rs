use crossbeam_channel::{bounded, Receiver, SendError, Sender};

#[derive(Debug)]
pub struct FileItem {
    pub name: String,
    pub modify_time: String,
    pub mode_str: String,
    pub size: String,
    pub is_dir: bool,
}

#[derive(Debug)]
pub enum UIEvent {
    StartLoading,
    Message(String),
    EndQueue,

    SwitchTab(usize),
    SetPath(String),
    InitColumn(Vec<Vec<FileItem>>),
    InitSelect(Vec<Option<usize>>),
    InitMark(Vec<Vec<usize>>),

    SetBookmark(Vec<String>),
    UpdateFileItem(Vec<FileItem>),
    ShowKeyNav(Vec<(String, String)>),
}

#[derive(Debug)]
pub enum EventBody {
    Single(UIEvent, Option<Sender<bool>>),
    Batch(Vec<UIEvent>, Option<Sender<bool>>),
    Queue(UIEvent, Option<Sender<bool>>),
}

pub struct UIEventSender(Sender<EventBody>);

pub type UIEventResult = Result<(), SendError<EventBody>>;

impl UIEventSender {
    pub fn new() -> (Self, Receiver<EventBody>) {
        let (tx, rx) = bounded(10);
        (UIEventSender(tx), rx)
    }

    pub fn start_loading(&self) -> UIEventResult {
        self.send(UIEvent::StartLoading)
    }

    pub fn send_sync(&self, event: UIEvent) -> UIEventResult {
        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Single(event, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn send(&self, event: UIEvent) -> UIEventResult {
        self.0.send(EventBody::Single(event, None))
    }

    pub fn batch_send_sync(&self, events: Vec<UIEvent>) -> UIEventResult {
        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Batch(events, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn batch_send(&self, events: Vec<UIEvent>) -> UIEventResult {
        self.0.send(EventBody::Batch(events, None))
    }

    pub fn queue_sync(&self, event: UIEvent) -> UIEventResult {
        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Queue(event, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn queue(&self, event: UIEvent) -> UIEventResult {
        self.0.send(EventBody::Queue(event, None))
    }

    pub fn end_queue(&self) -> UIEventResult {
        self.0.send(EventBody::Queue(UIEvent::EndQueue, None))
    }
}

impl Clone for UIEventSender {
    fn clone(&self) -> Self {
        UIEventSender(self.0.clone())
    }
}
