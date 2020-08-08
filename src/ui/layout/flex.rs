use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use delegate::delegate;

use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::collections::HashMap;
use std::ops::Deref;

pub struct Flex {
    drawable: Drawable,
    children: Vec<Mrc<dyn Draw>>,
    flex_count: u16,
    flex_children: HashMap<usize, u16>,
    vertical: bool,
}

impl Flex {
    pub fn new(vertical: bool) -> Self {
        Flex {
            drawable: Drawable::new(),
            children: Vec::new(),
            flex_count: 0,
            flex_children: HashMap::new(),
            vertical,
        }
    }

    pub fn add_flex<T: Draw + 'static>(&mut self, widget: Mrc<T>, flex: u16) {
        self.flex_count += &flex;
        self.children.push(widget);
        self.flex_children.insert(self.children.len() - 1, flex);
    }

    pub fn add<T: Draw + 'static>(&mut self, widget: Mrc<T>) {
        self.children.push(widget)
    }

    fn calc_self_size(&self, min: &Size, max: &Size, child_max: &Size) -> Size {
        if self.vertical {
            max.new_width(cmp::max(min.width, child_max.width))
        } else {
            max.new_height(cmp::max(min.height, child_max.height))
        }
    }

    fn calc_unit(&self, max: &Size, csum: &Size) -> (u16, u16) {
        let w = if self.vertical {
            max.height.saturating_sub(csum.height)
        } else {
            max.width.saturating_sub(csum.width)
        };
        let re = w % self.flex_count;
        ((w.saturating_sub(re)) / self.flex_count, re)
    }

    fn calc_remain_size(&self, max: &Size, csum: &Size) -> Size {
        if self.vertical {
            max.new_height(max.height.saturating_sub(csum.height))
        } else {
            max.new_width(max.width.saturating_sub(csum.width))
        }
    }

    fn calc_flex_ensure(&self, max: &Size, len: u16) -> (Size, Size) {
        if self.vertical {
            (Size::new(0, len), max.new_height(len))
        } else {
            (Size::new(len, 0), max.new_width(len))
        }
    }

    fn calc_next_point(&self, prev: &Rect) -> Point {
        if self.vertical {
            &prev.bottom_left() + (0i16, 1i16)
        } else {
            &prev.top_right() + (1i16, 0i16)
        }
    }

    fn ensure_non_flex(&mut self, _min: &Size, max: &Size) -> (Size, Size) {
        let mi = Size::zero();
        let mut cmax = Size::zero();
        let mut csum = Size::zero();

        self.children.iter().enumerate().for_each(|(idx, widget)| {
            if self.flex_children.contains_key(&idx) {
                return;
            }

            let si = widget
                .deref()
                .borrow_mut()
                .ensure(&mi, &self.calc_remain_size(max, &csum));
            csum += &si;
            cmax.keep_max(&si);
        });

        (cmax, csum)
    }
}

impl Draw for Flex {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn clear(&mut self);
            fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let (mut cmax, mut csum) = self.ensure_non_flex(min, max);
        if self.flex_count == 0 {
            let size = self.calc_self_size(min, max, &cmax);
            self.drawable.set_size(&size);
            return size;
        }

        let (unit, remain) = self.calc_unit(max, &csum);

        let mut x: Vec<_> = self
            .flex_children
            .iter()
            .map(|(idx, flex)| (&self.children[idx.clone()], unit * flex))
            .collect();

        if let Some(v) = x.last_mut() {
            v.1 += remain;
        }

        x.into_iter().for_each(|(it, width)| {
            let (m1, m2) = self.calc_flex_ensure(&max, width);
            let si = it.deref().borrow_mut().ensure(&m1, &m2);
            csum += &si;
            cmax.keep_max(&si);
        });

        let size = self.calc_self_size(min, max, &cmax);
        self.drawable.set_size(&size);
        return size;
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.children.iter().fold(None, |x, y| {
            if let Some(p) = x {
                y.deref().borrow_mut().move_to(&p);
            } else {
                y.deref().borrow_mut().move_to(point);
            }
            return Some(self.calc_next_point(y.deref().borrow().get_rect()));
        });
    }

    fn do_draw(&mut self) {
        self.children
            .iter()
            .for_each(|w| w.deref().borrow_mut().draw())
    }
}
