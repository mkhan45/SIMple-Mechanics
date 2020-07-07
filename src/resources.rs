use crate::{Point, Vector};
use crate::gui::imgui_wrapper::ImGuiWrapper;

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

pub type LuaRes = std::sync::Arc<std::sync::Mutex<rlua::Lua>>;

#[derive(Copy, Clone, Default)]
pub struct FPS(pub f64);

#[derive(Copy, Clone, Default)]
pub struct HiDPIFactor(pub f32);
