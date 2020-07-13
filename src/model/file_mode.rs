use std::iter::Map;
use std::collections::HashMap;

const MASK: u32 = 0o170000;
static TYPES: [(u32, char); 7] = [
    (0o140000, 's'), // socket
    (0o120000, 'l'), // symbolic link
    (0o100000, '-'), // regular file
    (0o060000, 'b'), // block device
    (0o040000, 'd'), // directory
    (0o020000, 'c'), // character device
    (0o010000, 'p'), // FIFO
];

fn find(mode: u32) -> char {
    let v = mode & MASK;
    match TYPES.iter().find(|it| { it.0 == v }) {
        Some(vv) => vv.1,
        None => '-'
    }
}

pub fn mode_string(mode: u32) -> String {
    let mut re = String::new();
    re.push(find(mode));

    "rwxrwxrwx".chars().enumerate().for_each(|(i, it)| {
        re.push(if mode & (1 << (8 - i)) != 0 { it } else { '-' })
    });
    re
}
