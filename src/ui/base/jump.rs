use crate::ui::base::shape::Point;

pub enum JumpType {
    OpenDir,
    Delete,
}

pub enum JumpInfo {
    OpenDir(String),
}

pub struct JumpPoint(Point, char, char, JumpInfo);
