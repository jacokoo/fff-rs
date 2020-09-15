use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::widget::label::Label;
use crate::ui::widget::line::Line;

pub struct CornerLine {
    line: Line,
    label: Label,
    corner_char: char,
    clear_char: char,
}

impl CornerLine {
    pub fn new(line_char: char, corner_char: char, clear_char: char) -> Self {
        CornerLine {
            line: Line::new_vertical(line_char),
            label: Label::from(corner_char.to_string()),
            corner_char,
            clear_char,
        }
    }
}

#[draw_to(line)]
impl Draw for CornerLine {
    fn do_ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.label.ensure(min, max);
        return self.line.ensure(min, max);
    }

    fn move_to(&mut self, point: &Point) {
        self.line.move_to(point);
        self.label.move_to(&(point.delta_y(-1)));
    }

    fn do_draw(&mut self) {
        self.line.draw();
        self.label.draw();
    }

    fn clear(&mut self) {
        self.line.clear();
        self.label.set_text(self.clear_char.to_string());
        self.label.draw();
        self.label.set_text(self.corner_char.to_string());
    }
}
