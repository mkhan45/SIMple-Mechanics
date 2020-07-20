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

#[derive(Copy, Clone)]
pub struct Resolution(pub Vector);
impl Default for Resolution {
    fn default() -> Self {
        Resolution(Vector::new(1920.0, 1080.0))
    }
}

#[derive(Copy, Clone, Default)]
pub struct DT(pub std::time::Duration);

#[derive(Clone, PartialEq, Debug)]
pub enum ShapeInfo {
    Rectangle(Option<Vector>),
    Circle(Option<f32>),
    Polygon(Option<Vec<Point>>),
    Polyline(Option<Vec<Point>>),
}

pub struct CreationData(pub Option<ShapeInfo>);
pub struct CreateShapeCentered(pub bool);
pub struct CreateShapeStatic(pub bool);

pub type LuaRes = std::sync::Arc<std::sync::Mutex<rlua::Lua>>;

#[derive(Copy, Clone, Default)]
pub struct FPS(pub f64);

#[derive(Copy, Clone, Default)]
pub struct HiDPIFactor(pub f32);

#[derive(Copy, Clone, Default)]
pub struct CreateMass(pub f32);

#[derive(Copy, Clone, Default)]
pub struct CreateElasticity(pub f32);

#[derive(Copy, Clone, Default)]
pub struct CreateFriction(pub f32);

#[derive(Copy, Clone, Default)]
pub struct FrameSteps(pub u16);

#[derive(Copy, Clone, Default)]
pub struct Paused(pub bool);

impl Paused {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}
