use crate::common::Functional;
use crate::ui::base::draw::Draw;
use crate::ui::base::shape::{Point, Size};
use crate::ui::event::UIEventSender;
use crate::ui::main::event_handle::handle;
use crossterm::style::{Color, Colors};
use crossterm::terminal::size;
use main::ui::UI;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;
use std::thread;

mod base;
pub mod event;
mod layout;
mod main;
mod widget;

pub type Mrc<T> = Rc<RefCell<T>>;

pub trait ToMrc: Sized {
    fn mrc(self) -> Mrc<Self> {
        Rc::new(RefCell::new(self))
    }
}

impl<T: Sized + Draw> ToMrc for T {}

pub trait ColorNone {
    fn none() -> Self;
}

impl ColorNone for Colors {
    fn none() -> Self {
        Colors::new(Color::Reset, Color::Reset)
    }
}

pub fn init_ui(tab_count: usize) -> UIEventSender {
    let (sender, rx) = UIEventSender::new();
    thread::spawn(move || {
        let (width, height) = size().unwrap();
        let size = Size::new(width, height);

        let ui = UI::new(tab_count).also_mut(|it| {
            it.ensure(&size, &size);
            it.move_to(&Point::new(0, 0));
            it.draw();
        });
        stdout().flush().unwrap();

        handle(ui, rx);
    });
    sender
}

//
// fn create_file(txt: &str, dir: bool) -> Mrc<FileLabel> {
//     FileLabel::new(txt)
//         .also_mut(|it| {
//             it.set_marked_color(Colors::new(Color::Yellow, Color::Black));
//             if dir {
//                 it.set_color(Colors::new(Color::Cyan, Color::Black));
//             } else {
//                 it.set_color(Colors::new(Color::White, Color::Black));
//             }
//         })
//         .mrc()
// }
//
// pub fn demo1() {
//     let (width, height) = size().unwrap();
//     let tab = Tab::new(
//         vec![
//             "1".to_string(),
//             "2".to_string(),
//             "3".to_string(),
//             "4".to_string(),
//         ],
//         0,
//     )
//     .mrc();
//
//     let path = PathIndicator::new("/Users/guyong/ws/rust/fff").mrc();
//
//     let top = Flex::row()
//         .also_mut(|it| {
//             it.add(tab.clone());
//             it.add(path.clone());
//             it.add_flex(Space::new().mrc(), 1);
//             it.add(Label::new("[?]").mrc());
//         })
//         .mrc();
//
//     let status_bar = Background::new(
//         Flex::row()
//             .also_mut(|it| {
//                 it.add(
//                     Label::new("status bar")
//                         .also_mut(|it| it.set_color(Colors::new(Color::Black, Color::Cyan)))
//                         .mrc(),
//                 );
//                 it.add_flex(Space::new().mrc(), 1);
//                 it.add(Spinner::new().mrc());
//             })
//             .mrc(),
//         Color::Cyan,
//     )
//     .mrc();
//
//     let board = Board::new()
//         .also_mut(|it| {
//             it.add_file_list(
//                 FileList::new()
//                     .also_mut(|it| {
//                         it.set_files(vec![
//                             create_file("hello", true),
//                             create_file("hello", false),
//                             create_file("hello world", false),
//                         ]);
//                         it.set_selected(Some(1));
//                         it.set_marked(vec![0, 1]);
//                     })
//                     .mrc(),
//             );
//
//             it.add_bookmark("Home".to_string());
//             it.add_bookmark("Root".to_string());
//         })
//         .mrc();
//
//     let mut root = Container::new(
//         Flex::column()
//             .also_mut(|it| {
//                 it.add(Padding::new(top.clone()).top_bottom(1).mrc());
//                 it.add_flex(SizedBox::new(board.clone()).max().mrc(), 1);
//                 it.add(SizedBox::new(status_bar.clone()).max_width().mrc());
//                 it.add(SizedBox::new(Space::new().mrc()).height(1).mrc());
//             })
//             .mrc(),
//     );
//
//     // let mut root = Container::new(tab.clone());
//
//     let size = Size::new(width, height);
//     root.ensure(&size, &size);
//     root.move_to(&Point::new(0, 0));
//     root.draw();
//
//     // println!("{:?}", tab.borrow().get_rect());
// }
