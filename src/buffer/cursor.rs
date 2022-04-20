use super::*;

pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

impl Positionable for Cursor {
    fn x(&self) -> i32 {
        return self.x;
    }
    fn y(&self) -> i32 {
        return self.y;
    }
}

impl Moveable for Cursor {
    fn move_down(&mut self) {
        self.y += 1;
    }
    fn move_up(&mut self) {
        self.y -= 1;
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
    fn move_right(&mut self) {
        self.x += 1;
    }
    fn mv(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
