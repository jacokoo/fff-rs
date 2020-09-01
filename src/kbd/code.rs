use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};


pub fn key_event_code(ev: &KeyEvent) -> String {
    let mut parts: Vec<String> = Vec::new();

    if ev.modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("ctrl".to_string());
    }

    if ev.modifiers.contains(KeyModifiers::ALT) {
        parts.push("alt".to_string())
    }

    let code = if let KeyCode::Char(c) = ev.code {
        c.to_string()
    } else if let KeyCode::F(n) = ev.code {
        format!("f{}", n)
    } else {
        (match ev.code {
            KeyCode::Tab => "tab",
            KeyCode::BackTab => "shift-tab",
            KeyCode::Backspace => "backspace",
            KeyCode::Delete => "delete",
            KeyCode::Down => "down",
            KeyCode::End => "end",
            KeyCode::Enter => "enter",
            KeyCode::Esc => "esc",
            KeyCode::Home => "home",
            KeyCode::Insert => "insert",
            KeyCode::Left => "left",
            KeyCode::PageDown => "pagedown",
            KeyCode::PageUp => "pageup",
            KeyCode::Right => "right",
            KeyCode::Up => "up",
            _ => "none",
        })
        .to_string()
    };
    parts.push(code);

    parts.join("-")
}
