use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::layout::flex::Flex;
use crate::ui::widget::label::Label;
use crate::ui::widget::quoted::Quoted;
use crate::ui::{Mrc, ToMrc};
use crossterm::style::{Color, Colors};

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

#[draw_to(label)]
impl Draw for TabItem {}

pub struct Tab {
    items: Vec<Mrc<TabItem>>,
    current: usize,
    main: Quoted,
}

impl Tab {
    pub fn new(strs: Vec<String>, current: usize) -> Self {
        let items: Vec<_> = strs.into_iter().map(|it| TabItem::new(it).mrc()).collect();
        items[current].borrow_mut().set_active(true);

        let flex = Flex::row().also_mut(|it| {
            for item in &items {
                it.add(item.clone());
            }
        });

        Tab {
            items,
            current,
            main: Quoted::new(flex.mrc()),
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

#[draw_to(main)]
impl Draw for Tab {}
