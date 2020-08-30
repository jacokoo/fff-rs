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
    Loading(bool),
    Message(String),

    SwitchTab(usize),
    SetPath(String),
    SetBookmark(Vec<String>),
    AddColumn(Vec<FileItem>),
}

#[derive(Debug)]
pub enum EventBody {
    Single(UIEvent, Option<Sender<bool>>),
    Batch(Vec<UIEvent>, Option<Sender<bool>>),
}

pub struct UIEventSender(Sender<EventBody>, bool);

impl UIEventSender {
    pub fn new() -> (Self, Receiver<EventBody>) {
        let (tx, rx) = bounded(1);
        (UIEventSender(tx, false), rx)
    }

    pub fn loading(&mut self) -> Result<(), SendError<EventBody>> {
        if self.1 {
            return Ok(());
        }

        self.1 = true;
        self.send_async(UIEvent::Loading(true))?;
        Ok(())
    }

    pub fn send_sync(&mut self, event: UIEvent) -> Result<(), SendError<EventBody>> {
        if self.1 {
            self.1 = false;
            return self.batch_send_sync(vec![UIEvent::Loading(false), event]);
        }

        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Single(event, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn send_async(&mut self, event: UIEvent) -> Result<(), SendError<EventBody>> {
        if self.1 {
            self.1 = false;
            return self.batch_send_async(vec![UIEvent::Loading(false), event]);
        }
        self.0.send(EventBody::Single(event, None))?;
        Ok(())
    }

    pub fn batch_send_sync(
        &mut self,
        mut events: Vec<UIEvent>,
    ) -> Result<(), SendError<EventBody>> {
        if self.1 {
            self.1 = false;
            events.insert(0, UIEvent::Loading(false));
        }
        let (tx, rx) = bounded(0);
        self.0.send(EventBody::Batch(events, Some(tx)))?;
        rx.recv().unwrap();
        Ok(())
    }

    pub fn batch_send_async(
        &mut self,
        mut events: Vec<UIEvent>,
    ) -> Result<(), SendError<EventBody>> {
        if self.1 {
            self.1 = false;
            events.insert(0, UIEvent::Loading(false));
        }

        self.0.send(EventBody::Batch(events, None))?;
        Ok(())
    }
}
