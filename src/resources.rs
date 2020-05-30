use crate::Vector;

#[derive(Copy, Clone)]
pub struct MousePos(pub Vector);

impl Default for MousePos {
    fn default() -> Self {
        MousePos(Vector::new(0.0, 0.0))
    }
}
