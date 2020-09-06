use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::event::FileItem;
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::bookmark::Bookmark;
use crate::ui::widget::corner_line::CornerLine;
use crate::ui::widget::file_column::FileColumn;
use crate::ui::widget::line::Line;
use crate::ui::{Mrc, ToMrc};

pub struct Board {
    main: Flex,
    column: Mrc<FileColumn>,
    bookmark: Mrc<Bookmark>,
    line: Mrc<CornerLine>,
}

impl Board {
    pub fn new() -> Self {
        let bookmark = Bookmark::new("BOOKMARKS".to_string()).mrc();
        let line = CornerLine::new('║', '╥', '─').mrc();
        let column = FileColumn::new().mrc();
        let items = Flex::row()
            .also_mut(|it| {
                it.add(bookmark.clone());
                it.add(SizedBox::new(line.clone()).max_height().mrc());
                it.add(column.clone());
            })
            .mrc();

        Board {
            main: Flex::column().also_mut(|it| {
                it.add(SizedBox::new(Line::new(false).mrc()).max_width().mrc());
                it.add_flex(items.clone(), 1);
            }),
            column,
            bookmark,
            line,
        }
    }

    pub fn set_bookmark(&mut self, bs: Vec<String>) {
        self.bookmark.borrow_mut().reset_items(bs);
    }

    pub fn add_bookmark(&mut self, txt: String) {
        self.bookmark.borrow_mut().add_item(txt);
    }

    pub fn init_files(&mut self, files: Vec<Vec<FileItem>>) {
        self.column.borrow_mut().init_file_list(files);
    }

    pub fn init_selected(&mut self, selected: Vec<Option<usize>>) {
        self.column.borrow_mut().init_selected(selected);
    }

    pub fn init_marked(&mut self, marks: Vec<Vec<usize>>) {
        self.column.borrow_mut().init_marked(marks);
    }

    pub fn refresh_files(&mut self, files: Vec<FileItem>) {
        self.column.borrow_mut().current_mut().set_files(files);
    }

    pub fn add_files(&mut self, files: Vec<FileItem>) {
        self.column.borrow_mut().add_file_list(files);
    }

    pub fn remove_files(&mut self, files: Option<Vec<FileItem>>) {
        self.column.borrow_mut().remove_file_list(files);
    }

    pub fn set_mark(&mut self, mark: Vec<usize>) {
        self.column.borrow_mut().current_mut().set_marked(mark);
    }

    pub fn set_selected(&mut self, selected: Option<usize>) {
        self.column
            .borrow_mut()
            .current_mut()
            .set_selected(selected);
    }

    pub fn set_show_detail(&mut self, show: bool) {
        self.column.borrow_mut().set_show_detail(show);
    }
}

#[draw_to(main)]
impl Draw for Board {}
