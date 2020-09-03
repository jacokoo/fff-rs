pub enum ViewMode {
    InColumn,
    InList,
}

pub struct Workspace {
    enter_path: String,
    home_path: String,
    root_path: String,
    current_group: usize,
    current_mode: ViewMode,
    show_detail: bool,
}
