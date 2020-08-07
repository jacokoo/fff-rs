use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::Mrc;
use delegate::delegate;
use num_integer::Integer;
use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::collections::HashMap;
use std::ops::Deref;

pub struct Row {
    drawable: Drawable,
    children: Vec<(Mrc<dyn Draw>, u16)>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            drawable: Drawable::new(),
            children: Vec::new(),
        }
    }

    pub fn add_flex<T: Draw + 'static>(&mut self, widget: Mrc<T>, flex: u16) {
        self.children.push((widget, flex))
    }

    pub fn add<T: Draw + 'static>(&mut self, widget: Mrc<T>) {
        self.add_flex(widget, 0)
    }
}

impl Draw for Row {
    delegate! {
        to self.drawable {
            fn get_rect(&self) -> &Rect;
            fn clear(&mut self);
            fn is_drawn(&self) -> bool;
            fn collect(&self, tp: JumpType) -> Option<Vec<JumpPoint>>;
        }
    }

    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        let mi = Size::new(0, 0);
        let mut map: HashMap<usize, u16> = HashMap::new();
        let mut w = 0u16;
        let mut h = 0u16;

        self.children
            .iter()
            .enumerate()
            .for_each(|(idx, (widget, flex))| {
                if flex > &0 {
                    map.insert(idx, flex.clone());
                    return;
                }
                let ma = Size::new(max.width - w, max.height);
                let si = widget.deref().borrow_mut().ensure(&mi, &ma);
                w += si.width;
                h = cmp::max(h, si.height);
            });

        let fs = map.values().fold(0u16, |acc, item| acc + item);
        if fs == 0 {
            let size = Size::new(max.width - w, h);
            self.drawable.set_size(&size);
            return size;
        }

        let (unit, remain) = (max.width - w).div_mod_floor(&fs);
        let len = map.len();

        let mut c = 0;
        self.children
            .iter()
            .enumerate()
            .for_each(|(idx, (widget, flex))| {
                if !map.contains_key(&idx) {
                    return;
                }

                c += 1;
                let ww = if c == len {
                    unit * flex + remain
                } else {
                    unit * flex
                };
                let m1 = min.new_width(ww);
                let m2 = max.new_width(ww);
                let si = widget.deref().borrow_mut().ensure(&m1, &m2);
                w += si.width;
                h = cmp::max(h, si.height);
            });
        let size = Size::new(max.width, cmp::max(min.height, h));
        self.drawable.set_size(&size);
        return size;
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.children.iter().fold(None, |x, (y, _)| {
            if let Some(p) = x {
                y.deref().borrow_mut().move_to(&p);
            } else {
                y.deref().borrow_mut().move_to(point);
            }
            return Some(&y.deref().borrow().get_rect().top_right() + (1, 0));
        });
    }

    fn do_draw(&mut self) {
        self.children
            .iter()
            .for_each(|(w, _)| w.deref().borrow_mut().draw())
    }
}
