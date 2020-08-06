use crate::ui::base::shape::Point;

pub enum JumpType {
    OPEN_DIR,
    DELETE,
}

pub enum JumpInfo {
    OPEN_DIR(String),
}

pub struct JumpPoint(Point, char, char, JumpInfo);
