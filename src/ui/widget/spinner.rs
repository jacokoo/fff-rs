use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crate::ui::widget::quoted::Quoted;
use crate::ui::ToMrc;
use crossterm::style::Colors;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};

const OK: &'static str = "✓";

pub struct Spinner {
    label: Arc<Mutex<Label>>,
    main: Quoted,
    started: Arc<RwLock<bool>>,
    chars: Vec<char>,
}

impl Spinner {
    pub fn new() -> Self {
        let label = Arc::new(Mutex::new(Label::new(OK)));
        Spinner {
            main: Quoted::new(Space::new().also_mut(|it| it.set(1, 1)).mrc()),
            label,
            started: Arc::new(RwLock::new(false)),
            chars: "-\\|/".chars().collect(),
        }
    }

    pub fn set_color(&mut self, color: Colors) {
        self.main.set_color(color.clone());
        self.label.lock().unwrap().set_color(color);
    }

    pub fn start(&mut self) {
        if *self.started.read().unwrap() {
            return;
        }

        {
            let mut s = self.started.write().unwrap();
            *s = true;
        }

        let lock = self.started.clone();
        let len = self.chars.len();
        let chars = self.chars.clone();
        let label = self.label.clone();
        thread::spawn(move || {
            let mut i = 0usize;
            while *lock.read().unwrap() {
                i += 1;
                i = i % len;

                Spinner::update_char(&label, chars[i].to_string());
                thread::sleep(time::Duration::from_millis(100));
            }
            Spinner::update_char(&label, OK.to_string());
        });
    }

    pub fn end(&mut self) {
        let mut s = self.started.write().unwrap();
        *s = false;
    }

    fn update_char(label: &Arc<Mutex<Label>>, c: String) {
        let mut it = label.lock().unwrap();
        it.set_text(c);
        it.clear();
        it.draw();
        stdout().flush().unwrap();
    }
}

#[draw_to(main)]
impl Draw for Spinner {
    fn move_to(&mut self, point: &Point) {
        self.main.move_to(point);
        let mut l = self.label.lock().unwrap();
        l.move_to(&(point + (1, 0)));
    }

    fn do_ensure(&mut self, min: &Size, max: &Size) -> Size {
        let mut l = self.label.lock().unwrap();
        l.ensure(min, max);
        self.main.ensure(min, max)
    }

    fn do_draw(&mut self) {
        self.main.draw();
        let mut l = self.label.lock().unwrap();
        l.draw();
    }
}
