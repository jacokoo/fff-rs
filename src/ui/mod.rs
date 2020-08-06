use crate::ui::base::draw::Draw;
use crate::ui::base::label::Label;
use crate::ui::base::shape::{Point, Size};

mod base;
pub mod spin;

pub fn demo() {
    let mut label = Label::new("ℝℝℝℝℝ".to_string());
    label.move_to(&Point::new(0, 0));
    label.ensure(&Size::new(10, 100));
    label.draw();

    println!("{}", label.get_rect().get_width());
}
