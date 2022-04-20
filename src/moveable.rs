use crate::positionable::Positionable;
pub trait Moveable: Positionable {
    fn move_down(&mut self);
    fn move_up(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn mv(&mut self, x: i32, y: i32);
}
