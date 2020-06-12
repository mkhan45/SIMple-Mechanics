use crate::{Point, Vector};
use ncollide2d;

#[derive(Copy, Clone)]
pub struct MousePos(pub Vector);

impl Default for MousePos {
    fn default() -> Self {
        MousePos(Vector::new(0.0, 0.0))
    }
}

#[derive(Copy, Clone, Default)]
pub struct DT(pub std::time::Duration);

#[derive(Copy, Clone)]
pub enum ShapeInfo {
    Rectangle(Vector),
    Circle(f32),
}

#[derive(Copy, Clone)]
pub struct CreateShapeData {
    pub shape: ShapeInfo,
    pub pos: Point,
    pub centered: bool,
}

#[derive(Copy, Clone)]
pub struct CreationData {
    creating: bool,
    shape_data: Option<CreateShapeData>,
}

impl Default for CreationData {
    fn default() -> Self {
        CreationData {
            creating: false,
            shape_data: None,
        }
    }
}
