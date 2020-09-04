use crate::ui::base::draw::Draw;

use crate::common::Functional;
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::event::FileItem;
use crate::ui::layout::background::Background;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crate::ui::{ColorNone, Mrc, ToMrc};
use crossterm::style::{Color, Colors};
use std::ops::Deref;

trait InverseColor {
    fn inverse(&self) -> Self;
}

impl InverseColor for Colors {
    fn inverse(&self) -> Self {
        Colors::new(
            self.background.unwrap_or(Color::Reset),
            self.foreground.unwrap_or(Color::Reset),
        )
    }
}

pub struct FileLabel {
    labels: Vec<Mrc<Label>>,
    selected: bool,
    marked: bool,
    color: Colors,
    marked_color: Colors,
    marker: Label,
    background: Background,
    item: FileItem,
    pub show_detail: bool,
    max: usize,
}

impl FileLabel {
    pub fn new(item: FileItem, max: usize) -> Self {
        let c = if item.is_dir {
            Colors::new(Color::Cyan, Color::Black)
        } else {
            Colors::new(Color::White, Color::Black)
        };

        let (labels, body) = FileLabel::create_body(false, max, &item);
        FileLabel {
            selected: false,
            marked: false,
            color: c,
            marked_color: Colors::new(Color::Yellow, Color::Black),
            marker: Label::new("*"),
            background: Background::new(body, c.background.unwrap()),
            item,
            labels,
            show_detail: false,
            max,
        }
    }

    pub fn set_color(&mut self, c: Colors) {
        self.color = c;
        self.ensure_color();
    }

    pub fn set_selected(&mut self, s: bool) {
        self.selected = s;
        self.ensure_color();
    }

    pub fn set_marked(&mut self, s: bool) {
        self.marked = s;
        self.ensure_color();
    }

    pub fn set_marked_color(&mut self, c: Colors) {
        self.marked_color = c;
        self.ensure_color();
    }

    pub fn set_show_detail(&mut self, show: bool) {
        self.show_detail = show;
        let (labels, flex) = FileLabel::create_body(show, self.max, &self.item);
        self.labels = labels;
        self.background.set_child(flex);
        self.ensure_color();
    }

    fn ensure_color(&mut self) {
        let mut used_color = if self.marked {
            self.marked_color
        } else {
            self.color
        };

        if self.selected {
            used_color = used_color.inverse();
        }

        self.labels.iter().for_each(|it| {
            it.deref().borrow_mut().set_color(used_color.clone());
        });
        self.marker.set_color(used_color.clone());
        self.background.set_color(used_color);
    }

    fn create_body(show_detail: bool, max: usize, item: &FileItem) -> (Vec<Mrc<Label>>, Mrc<Flex>) {
        let mut ls = Vec::new();
        let mut flex = Flex::row();
        flex = if !show_detail {
            flex.also_mut(|it| {
                let l = Label::new(&item.name).mrc();
                let l2 = Label::new(&item.size).mrc();

                ls.push(l.clone());
                ls.push(l2.clone());

                it.add(Space::new_with_width(2).mrc());
                it.add(l);
                it.add_flex(Space::new_with_width(2).mrc(), 1);
                it.add(l2);
                it.add(Space::new_with_width(2).mrc());
            })
        } else {
            flex.also_mut(|it| {
                let l1 = Label::from(format!(
                    "{0} {1} {2:>3$} {4}",
                    &item.modify_time, &item.mode_str, &item.size, max, &item.name
                ))
                .mrc();

                ls.push(l1.clone());

                it.add(Space::new_with_width(2).mrc());
                it.add(l1);
                it.add(Space::new_with_width(2).mrc());
            })
        };
        (ls, flex.mrc())
    }
}

#[draw_to(background)]
impl Draw for FileLabel {
    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.marker.ensure(min, max);
        self.background.ensure(min, max)
    }

    fn do_draw(&mut self) {
        self.background.draw();
        if self.marked {
            self.marker
                .move_to(&(&self.background.get_rect().top_left() + (1, 0)));
            self.marker.draw()
        }
    }
}
