use crate::ui::base::draw::Draw;

use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::container::UseMin;
use crate::ui::layout::flex::Flex;

use crate::ui::layout::space::Space;
use crate::ui::widget::file_label::FileLabel;
use crate::ui::widget::label::Label;
use crate::ui::{Functional, Mrc, ToMrc};

pub struct FileList {
    files: Vec<Mrc<FileLabel>>,
    flex: Flex,
    select_index: Option<usize>,
    marked: Vec<usize>,
}

impl FileList {
    pub fn new() -> Self {
        FileList {
            files: Vec::new(),
            flex: Flex::column().also_mut(|it| it.set_stretch()),
            select_index: None,
            marked: Vec::new(),
        }
    }

    pub fn set_files(&mut self, fs: Vec<Mrc<FileLabel>>) {
        self.files = fs;
        self.set_selected(None);
        self.set_marked(Vec::new());
    }

    pub fn set_selected(&mut self, selected: Option<usize>) {
        if let Some(s) = self.select_index {
            self.files[s].borrow_mut().set_selected(false);
        }

        self.select_index = selected;
        if let Some(s) = self.select_index {
            self.files[s].borrow_mut().set_selected(true)
        }
    }

    pub fn set_marked(&mut self, marked: Vec<usize>) {
        self.marked.iter().for_each(|it| {
            self.files[it.clone()].borrow_mut().set_marked(false);
        });

        self.marked = marked;
        self.marked.iter().for_each(|it| {
            self.files[it.clone()].borrow_mut().set_marked(true);
        })
    }

    fn prepare_ensure(&mut self, height: usize) {
        self.flex.empty_it();
        let h = height - 1;
        for (idx, file) in self.files.iter().enumerate() {
            if idx >= h {
                break;
            }
            self.flex.add(file.clone());
        }
        self.flex.add_flex(Space::new().mrc(), 1);
        self.flex.add(
            UseMin::width(
                Flex::row()
                    .also_mut(|it| {
                        it.add_flex(Space::new().mrc(), 1);
                        if let Some(idx) = self.select_index {
                            it.add(
                                Label::from(format!("{}/{}  ", idx + 1, self.files.len())).mrc(),
                            );
                        }
                    })
                    .mrc(),
            )
            .mrc(),
        );
    }
}

#[draw_to(flex)]
impl Draw for FileList {
    fn ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.prepare_ensure(max.height as usize);
        let s = self.flex.ensure(min, max);
        return s;
    }
}
