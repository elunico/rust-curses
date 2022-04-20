pub use crate::moveable::Moveable;
pub use crate::positionable::Positionable;

impl Positionable for pancurses::Window {
    fn x(&self) -> i32 {
        return self.get_cur_x();
    }
    fn y(&self) -> i32 {
        return self.get_cur_y();
    }
}

impl Moveable for pancurses::Window {
    fn mv(&mut self, x: i32, y: i32) {
        pancurses::Window::mv(self, x, y);
    }
    fn move_down(&mut self) {
        self.mv(self.y() + 1, self.x());
    }
    fn move_up(&mut self) {
        self.mv(self.y() - 1, self.x());
    }
    fn move_left(&mut self) {
        self.mv(self.y(), self.x() - 1);
    }
    fn move_right(&mut self) {
        self.mv(self.y(), self.x() + 1);
    }
}
