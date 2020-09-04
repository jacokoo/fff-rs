use crate::ui::base::draw::{Draw, Drawable};

use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use crossterm::style::{Color, Colors};

pub struct Background {
    drawable: Drawable,
    color: Color,
}

impl Background {
    pub fn new<T: Draw + 'static>(child: Mrc<T>, color: Color) -> Self {
        Background {
            drawable: Drawable::new_with_child(child),
            color,
        }
    }

    pub fn set_color(&mut self, color: Colors) {
        self.color = color.background.unwrap_or(Color::Reset);
    }

    pub fn set_child<T: Draw + 'static>(&mut self, child: Mrc<T>) {
        self.drawable = Drawable::new_with_child(child);
    }
}

#[draw_to(drawable)]
impl Draw for Background {
    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let s = self.drawable.mut_child().ensure(min, max);
        self.drawable.set_size(&s);
        s
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.drawable.mut_child().move_to(point);
    }

    fn do_draw(&mut self) {
        self.clear();
        self.drawable.mut_child().draw();
    }

    fn clear(&mut self) {
        self.get_rect().clear_with_color(Some(self.color));
    }
}
