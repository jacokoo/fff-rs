use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::jump::{JumpPoint, JumpType};
use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::flex::Flex;
use crate::ui::widget::label::Label;
use crate::ui::{Functional, Mrc, ToMrc};
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::QueueableCommand;
use delegate::delegate;

struct TabItem {
    active: bool,
    label: Label,
    color: Colors,
}

impl TabItem {
    fn format(str: String) -> String {
        format!(" {} ", str)
    }

    fn new(str: String) -> TabItem {
        TabItem {
            active: false,
            label: Label::from(TabItem::format(str)),
            color: Colors::new(Color::Black, Color::Cyan),
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
        if self.active {
            self.label.set_color(self.color.clone());
        } else {
            self.label.reset_color();
        }
    }

    pub fn set_text(&mut self, txt: String) {
        self.label.set_text(TabItem::format(txt))
    }
}

impl Draw for TabItem {
    delegate! {
        to self.label {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn ensure(&mut self, min: &Size, max: &Size) -> Size;
            fn do_draw(&mut self);
            fn clear(&mut self);
        }
    }
}

pub struct Tab {
    items: Vec<Mrc<TabItem>>,
    current: usize,
    flex: Flex,
}

impl Tab {
    pub fn new(strs: Vec<String>, current: usize) -> Self {
        let items: Vec<_> = strs.into_iter().map(|it| TabItem::new(it).mrc()).collect();
        items[current].borrow_mut().set_active(true);

        let flex = Flex::new(false).also_mut(|it| {
            it.add(Label::new("[").mrc());
            for item in &items {
                it.add(item.clone());
            }
            it.add(Label::new("]").mrc());
        });

        Tab {
            items,
            current,
            flex,
        }
    }

    pub fn set_active(&mut self, current: usize) {
        self.items[self.current].borrow_mut().set_active(false);
        self.items[current].borrow_mut().set_active(true);
        self.current = current;

        self.clear();
        self.draw();
    }
}

impl Draw for Tab {
    delegate! {
        to self.flex {
            fn get_rect(&self) -> &Rect;
            fn move_to(&mut self, point: &Point);
            fn ensure(&mut self, min: &Size, max: &Size) -> Size;
            fn do_draw(&mut self);
            fn clear(&mut self);
        }
    }
}
