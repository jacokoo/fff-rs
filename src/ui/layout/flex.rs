use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::{InnerFunctional, Mrc};
use std::cmp;
use std::collections::HashMap;
use std::ops::Deref;

pub struct Flex {
    drawable: Drawable,
    flex_count: u16,
    flex_children: HashMap<usize, u16>,
    vertical: bool,
    stretch: bool,
    expanded: bool,
}

impl Flex {
    pub fn row() -> Self {
        Self::new(false)
    }

    pub fn column() -> Self {
        Self::new(true)
    }

    fn new(vertical: bool) -> Self {
        Flex {
            drawable: Drawable::new(),
            flex_count: 0,
            flex_children: HashMap::new(),
            vertical,
            stretch: false,
            expanded: true,
        }
    }

    pub fn add_flex<T: Draw + 'static>(&mut self, widget: Mrc<T>, flex: u16) {
        self.flex_count += &flex;
        self.drawable.children.push(widget);
        self.flex_children
            .insert(self.drawable.children.len() - 1, flex);
    }

    pub fn add<T: Draw + 'static>(&mut self, widget: Mrc<T>) {
        self.drawable.children.push(widget);
    }

    pub fn empty_it(&mut self) {
        self.drawable.children.clear();
        self.flex_children.clear();
        self.flex_count = 0;
    }

    pub fn set_stretch(&mut self) {
        self.stretch = true;
    }

    fn calc_self_size(&self, min: &Size, _max: &Size, child_max: &Size, child_sum: &Size) -> Size {
        if self.vertical {
            Size::new(
                cmp::max(min.width, child_max.width),
                cmp::max(min.height, child_sum.height),
            )
        } else {
            Size::new(
                cmp::max(min.width, child_sum.width),
                cmp::max(min.height, child_max.height),
            )
        }
    }

    fn before_end_ensure(
        &mut self,
        min: &Size,
        max: &Size,
        child_max: &Size,
        child_sum: &Size,
    ) -> Size {
        let size = self.calc_self_size(min, max, child_max, child_sum);
        self.drawable.set_size(&size);
        if !self.stretch {
            return size;
        }

        self.drawable.for_each(|mut it| {
            if self.vertical {
                if it.get_rect().get_width() < size.width {
                    let s = size.new_height(it.get_rect().get_height());
                    it.ensure(&s, &s);
                }
            } else {
                if it.get_rect().get_height() < size.height {
                    let s = size.new_width(it.get_rect().get_width());
                    it.ensure(&s, &s);
                }
            }
        });

        return size;
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
            prev.bottom_left().delta_y(1)
        } else {
            prev.top_right().delta_x(1)
        }
    }

    fn ensure_non_flex(&mut self, _min: &Size, max: &Size) -> (Size, Size) {
        let mi = Size::zero();
        let mut cmax = Size::zero();
        let mut csum = Size::zero();

        self.drawable.for_each_indexed(|idx, mut widget| {
            if self.flex_children.contains_key(&idx) {
                return;
            }

            let si = widget.ensure(&mi, &self.calc_remain_size(max, &csum));
            csum += &si;
            cmax.keep_max(&si);
        });

        (cmax, csum)
    }

    // only for file column
    pub fn pop(&mut self) {
        let idx = self.drawable.children.len() - 1;
        if let Some(v) = self.drawable.children.pop() {
            if self.flex_children.contains_key(&idx) {
                self.flex_children.remove(&idx);
                self.flex_count -= 1;
            }
            v.deref().borrow_mut().clear();
        }
    }

    // only for file column
    pub fn push<T: Draw + 'static>(&mut self, widget: Mrc<T>) {
        let last = self.last();
        if last.is_none() {
            return;
        }

        let (min, max) = last.unwrap();
        let w = self.drawable.children.iter().fold(0, |acc, it| {
            acc + it.deref().borrow().get_rect().get_width()
        });

        let ma = max.new_width(max.width - w - 1);
        widget.inner_apply(|mut ww| {
            ww.ensure(&min, &ma);
            ww.move_to(&(self.get_rect().top_left().delta_x(w as i16)));
            ww.draw();
        });
        self.drawable.children.push(widget);
    }
}

#[draw_to(drawable)]
impl Draw for Flex {
    fn do_ensure(&mut self, min: &Size, max: &Size) -> Size {
        let (mut cmax, mut csum) = self.ensure_non_flex(min, max);
        if self.flex_count == 0 {
            return self.before_end_ensure(min, max, &cmax, &csum);
        }

        let (unit, remain) = self.calc_unit(max, &csum);

        let mut x: Vec<_> = self
            .flex_children
            .iter()
            .map(|(idx, flex)| (&self.drawable.children[idx.clone()], unit * flex))
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

        return self.before_end_ensure(min, max, &cmax, &csum);
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.drawable.fold(None, |x, mut y| {
            if let Some(p) = x {
                y.move_to(&p);
            } else {
                y.move_to(point);
            }
            return Some(self.calc_next_point(y.get_rect()));
        });
    }

    fn do_draw(&mut self) {
        self.drawable.for_each(|mut it| it.draw());
    }
}
