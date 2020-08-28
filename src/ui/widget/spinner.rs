use std::{thread, time};
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

use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Rect, Size};

use crate::ui::widget::label::Label;
use crate::ui::widget::quoted::Quoted;
use crate::ui::{Functional, Mrc, ToMrc};
use std::borrow::BorrowMut;
use std::io::{stdout, Write};
use std::ops::Deref;

const OK: &'static str = "✓";

pub struct Spinner {
    label: Mrc<Label>,
    main: Quoted,
    started: bool,
    chars: Vec<char>,
}

impl Spinner {
    pub fn new() -> Self {
        let label = Label::new(OK).mrc();
        Spinner {
            main: Quoted::new(label.clone()),
            label,
            started: false,
            chars: "▁▃▄▅▆▇█▇▆▅▄▃▁".chars().collect(),
        }
    }

    pub fn start(&mut self) {
        if self.started {
            return;
        }

        self.started = true;
        let mut i = 0usize;
        let len = self.chars.len();
        while self.started {
            i += 1;
            i = i % len;
            self.label.deref().borrow_mut().also_mut(|it| {
                it.set_text(self.chars[i].to_string());
                // it.clear();
                it.draw();
                stdout().flush().unwrap();
            });
            thread::sleep(time::Duration::from_millis(200));
        }
    }

    pub fn end(&mut self) {
        self.started = false;
        self.label.deref().borrow_mut().also_mut(|it| {
            it.set_text(OK.to_string());
            it.clear();
            it.draw();
        });
    }
}

#[draw_to(main)]
impl Draw for Spinner {}
