use crate::common::Functional;
use crate::ui::base::draw::{Draw, Drawable};
use crate::ui::base::shape::{Point, Size};
use crate::ui::event::FileItem;
use crate::ui::layout::container::UseMin;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::space::Space;
use crate::ui::main::corner_line::CornerLine;
use crate::ui::main::file_label::FileLabel;
use crate::ui::widget::label::Label;
use crate::ui::{InnerFunctional, Mrc, ToMrc};

pub struct FileList {
    drawable: Drawable,
    files: Vec<Mrc<FileLabel>>,
    flex: Flex,
    select_index: Option<usize>,
    marked: Vec<usize>,
    line: CornerLine,
    show_detail: bool,
    indicator: Mrc<Label>,
    indicator_line: Mrc<UseMin>,
}

impl FileList {
    pub fn new(show_detail: bool) -> Self {
        let indicator = Label::new("").mrc();
        FileList {
            drawable: Drawable::new(),
            files: Vec::new(),
            flex: Flex::column().also(|it| it.set_stretch()),
            select_index: None,
            marked: Vec::new(),
            line: CornerLine::new('│', '┬', '─'),
            show_detail,
            indicator_line: UseMin::width(
                Flex::row()
                    .also(|it| {
                        it.add_flex(Space::new().mrc(), 1);
                        it.add(indicator.clone());
                    })
                    .mrc(),
            )
            .mrc(),
            indicator,
        }
    }

    pub fn set_show_detail(&mut self, show: bool) {
        self.show_detail = show;
        self.files.iter().for_each(|it| {
            it.borrow_mut().set_show_detail(show);
        });
    }

    pub fn set_files(&mut self, list: Vec<FileItem>) {
        let max = list
            .iter()
            .fold(0usize, |acc, it| std::cmp::max(acc, it.size.len()));
        let files: Vec<_> = list
            .into_iter()
            .map(|it| FileLabel::new(it, max, self.show_detail).mrc())
            .collect();
        self.files = files;
        self.set_marked(Vec::new());
        self.set_selected(None);

        self.redraw();
    }

    pub fn set_selected(&mut self, selected: Option<usize>) {
        if let Some(s) = self.select_index {
            self.files[s].borrow_mut().set_selected(false);
            self.files[s].borrow_mut().redraw();
        }

        self.select_index = selected;
        if let Some(s) = self.select_index {
            self.files[s].borrow_mut().set_selected(true);
            self.files[s].borrow_mut().redraw();
            self.indicator
                .borrow_mut()
                .set_text(format!("{}/{}  ", s + 1, self.files.len()));
            self.indicator_line.borrow_mut().redraw();
        }
    }

    pub fn set_marked(&mut self, marked: Vec<usize>) {
        self.marked.iter().for_each(|it| {
            self.files[it.clone()].inner_apply(|mut i| {
                i.set_marked(false);
                i.redraw();
            });
        });

        self.marked = marked;
        self.marked.iter().for_each(|it| {
            self.files[it.clone()].inner_apply(|mut i| {
                i.set_marked(true);
                i.redraw();
            });
        });
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
        self.flex.add(self.indicator_line.clone());
    }
}

#[draw_to(drawable)]
impl Draw for FileList {
    fn do_ensure(&mut self, min: &Size, max: &Size) -> Size {
        self.prepare_ensure(max.height as usize);
        let mm = Size::new(if self.show_detail { max.width } else { 30 }, max.height);
        let s = self.flex.ensure(min, &mm);
        self.line
            .ensure(&min.new_height(s.height), &mm.new_width(1));
        let ss = s.new_width(s.width + 1);
        self.drawable.set_size(&ss);
        return ss;
    }

    fn move_to(&mut self, point: &Point) {
        self.drawable.move_to(point);
        self.flex.move_to(point);
        self.line
            .move_to(&(point.delta_x(self.flex.get_rect().get_width() as i16)))
    }

    fn clear(&mut self) {
        self.line.clear();
        self.drawable.clear();
    }

    fn do_draw(&mut self) {
        self.flex.draw();
        self.line.draw();
    }
}
