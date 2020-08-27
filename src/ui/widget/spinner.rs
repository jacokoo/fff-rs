// use std::{thread, time};
// use termion::{clear, color, cursor};
//
// pub fn spin() {
//     let chars: Vec<char> = "▁▃▄▅▆▇█▇▆▅▄▃▁".chars().collect();
//     let mut i = 0;
//     print!("{}", cursor::Hide);
//     while i < 100 {
//         println!(
//             "{}{}{}{}",
//             cursor::Goto(1, 1),
//             clear::AfterCursor,
//             color::Fg(color::Red),
//             chars[i % chars.len()]
//         );
//         thread::sleep(time::Duration::from_millis(200));
//         i += 1;
//     }
//     print!("{}", cursor::Show);
// }


use crate::ui::widget::label::Label;
use crate::ui::widget::quoted::Quoted;
use crate::ui::{Mrc, ToMrc};


const CHARS: &'static str = "▁▃▄▅▆▇█▇▆▅▄▃▁";
const OK: &'static str = "☑";

pub struct Spinner {
    label: Mrc<Label>,
    main: Quoted,
}

impl Spinner {
    pub fn new() -> Self {
        let label = Label::new(OK).mrc();
        Spinner {
            main: Quoted::new(label.clone()),
            label,
        }
    }
}
