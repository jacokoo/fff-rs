use std::{thread, time};
use termion::{clear, color, cursor};

pub fn spin() {
    let chars: Vec<char> = "▁▃▄▅▆▇█▇▆▅▄▃▁".chars().collect();
    let mut i = 0;
    print!("{}", cursor::Hide);
    while i < 100 {
        println!(
            "{}{}{}{}",
            cursor::Goto(1, 1),
            clear::AfterCursor,
            color::Fg(color::Red),
            chars[i % chars.len()]
        );
        thread::sleep(time::Duration::from_millis(200));
        i += 1;
    }
    print!("{}", cursor::Show);
}
