use crate::gui::imgui_wrapper::ImGuiWrapper;
use crate::{Point, Vector};

#[derive(Copy, Clone)]
pub struct MousePos(pub Vector);

#[derive(Copy, Clone)]
pub struct MouseStartPos(pub Option<Vector>);

impl Default for MousePos {
    fn default() -> Self {
        MousePos(Vector::new(0.0, 0.0))
    }
}

#[derive(Copy, Clone, Default)]
pub struct DT(pub std::time::Duration);

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShapeInfo {
    Rectangle(Option<Vector>),
    Circle(Option<f32>),
}

#[derive(Copy, Clone, Debug)]
pub struct CreateShapeData {
    pub shape: ShapeInfo,
    pub centered: bool,
}

pub struct CreationData(pub Option<CreateShapeData>);

pub type LuaRes = std::sync::Arc<std::sync::Mutex<rlua::Lua>>;

#[derive(Copy, Clone, Default)]
pub struct FPS(pub f64);

#[derive(Copy, Clone, Default)]
pub struct HiDPIFactor(pub f32);
