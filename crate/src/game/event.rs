use crate::game::controls::keyboard::Key;

pub enum Event {
  MouseDown(i32, i32),
  KeyDown(Key),
  KeyUp(Key)
}