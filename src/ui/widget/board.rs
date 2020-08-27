use crate::ui::base::draw::{Draw};

use crate::ui::base::shape::{Point, Rect, Size};
use crate::ui::layout::flex::Flex;
use crate::ui::layout::sized::SizedBox;
use crate::ui::widget::bookmark::Bookmark;
use crate::ui::widget::file_column::{CornerLine, FileColumn};
use crate::ui::widget::file_list::FileList;
use crate::ui::widget::line::Line;
use crate::ui::{Functional, Mrc, ToMrc};

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

    pub fn add_bookmark(&mut self, txt: String) {
        self.bookmark.borrow_mut().add_item(txt);
    }

    pub fn add_file_list(&mut self, list: Mrc<FileList>) {
        self.column.borrow_mut().add_file_list(list);
    }
}

draw_to! {
    Board.main
}
