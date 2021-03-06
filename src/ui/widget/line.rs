use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::shape::Size;
use crate::ui::ColorNone;
use crossterm::style::{Colors, Print, SetColors};
use crossterm::QueueableCommand;
use std::cmp;
use std::io::stdout;

pub struct Line {
    drawable: Drawable,
    vertical: bool,
    vchar: char,
    hchar: char,
    colors: Colors,
}

impl Line {
    pub fn new(vertical: bool) -> Self {
        Line::new_with_char(vertical, '│', '─')
    }

    pub fn new_vertical(char: char) -> Self {
        Line::new_with_char(true, char, ' ')
    }

    pub fn new_horizontal(char: char) -> Self {
        Line::new_with_char(false, ' ', char)
    }

    pub fn set_color(mut self, colors: Colors) -> Self {
        self.colors = colors;
        self
    }

    fn new_with_char(vertical: bool, vchar: char, hchar: char) -> Self {
        Line {
            drawable: Drawable::new(),
            vertical,
            vchar,
            hchar,
            colors: Colors::none(),
        }
    }
}

#[draw_to(drawable)]
impl Draw for Line {
    fn do_ensure(&mut self, min: &Size, _max: &Size) -> Size {
        let s = if self.vertical {
            min.new_width(cmp::max(min.width, 1))
        } else {
            min.new_height(cmp::max(min.height, 1))
        };
        self.drawable.set_size(&s);
        s
    }

    fn do_draw(&mut self) {
        if self.vertical {
            let mut io = stdout();
            let p = self.get_rect().top_left();

            io.queue(SetColors(self.colors)).unwrap();

            for i in 0..self.get_rect().get_height() {
                io.queue((p.delta_y(i as i16)).cursor())
                    .unwrap()
                    .queue(Print(self.vchar))
                    .unwrap();
            }
        } else {
            let mut s = String::new();
            for _i in 0..self.get_rect().get_width() {
                s.push(self.hchar);
            }

            stdout()
                .queue(self.get_rect().top_left().cursor())
                .unwrap()
                .queue(SetColors(self.colors))
                .unwrap()
                .queue(Print(s))
                .unwrap();
        }
    }
}
