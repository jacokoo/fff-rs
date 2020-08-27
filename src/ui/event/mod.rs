use tokio::sync::oneshot::Sender;


pub struct FileItem {
    pub name: String,
    pub detail: String,
    pub size: String,
    pub is_dir: bool,
}

pub enum EventData {
    CurrentTab(usize),
    SetPath(String),
    SetBookmark(Vec<String>),
    Message(String),
    AddColumn(Vec<FileItem>),
}

pub enum Event {
    Single(EventData, Sender<bool>),
    Batch(Vec<EventData>, Sender<bool>),
}
