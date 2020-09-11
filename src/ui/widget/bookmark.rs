use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::Size;
use crate::ui::layout::center::Center;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::padding::Padding;
use crate::ui::layout::sized::SizedBox;
use crate::ui::layout::space::Space;
use crate::ui::widget::label::Label;
use crate::ui::widget::line::Line;
use crate::ui::{Mrc, ToMrc};

pub struct Bookmark {
    items: Vec<Mrc<Label>>,
    main: Flex,
    list: Mrc<Flex>,
}

impl Bookmark {
    pub fn new(title: String) -> Self {
        let list = Flex::column()
            .also_mut(|it| {
                it.set_stretch();
            })
            .mrc();
        Bookmark {
            items: Vec::new(),
            main: Flex::column().also_mut(|it| {
                it.set_stretch();
                it.add(
                    Padding::new(Center::new(Label::new(&title).mrc()).mrc())
                        .top_bottom(1)
                        .left_right(2)
                        .mrc(),
                );
                it.add(SizedBox::new(Line::new(false).mrc()).mrc());
                it.add(Padding::new(list.clone()).left_right(2).mrc());
                it.add_flex(Space::new().mrc(), 1);
            }),
            list,
        }
    }

    pub fn add_item(&mut self, txt: String) {
        self.items.push(Label::from(txt).mrc());
    }

    pub fn reset_items(&mut self, bs: Vec<String>) {
        self.items.clear();
        bs.into_iter().for_each(|b| self.add_item(b));
    }

    fn prepare_ensure(&mut self, height: u16) {
        self.list.borrow_mut().empty_it();
        for (idx, item) in self.items.iter().enumerate() {
            if idx >= height as usize {
                break;
            }
            self.list.borrow_mut().add(item.clone());
        }
    }
}

#[draw_to(main)]
impl Draw for Bookmark {
    fn do_ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.prepare_ensure(max.height);
        return self.main.ensure(min, max);
    }
}
