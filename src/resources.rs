use crate::{Point, Vector};
use ggez::graphics::Rect;

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
#[allow(dead_code)]
pub enum ShapeInfo {
    Rectangle(Option<Vector>),
    Circle(Option<f32>),
    Polygon(Option<Vec<Point>>),
    Polyline(Option<Vec<Point>>),
}

pub struct CreationData(pub Option<ShapeInfo>);
pub struct CreateShapeCentered(pub bool);
impl CreateShapeCentered {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

pub struct CreateShapeStatic(pub bool);
impl CreateShapeStatic {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

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

#[derive(Copy, Clone, Default)]
pub struct GraphMinMax(pub f32, pub f32);

#[derive(Copy, Clone)]
pub struct GraphPosData(pub Rect);

impl Default for GraphPosData {
    fn default() -> Self {
        GraphPosData(Rect::new(00.0, 0.0, 10.0, 10.0))
    }
}

#[derive(Copy, Clone, Default)]
pub struct MovingGraph(pub bool);

#[derive(Copy, Clone, Default)]
pub struct ScalingGraph(pub bool);

#[derive(Copy, Clone, Default)]
pub struct Selected(pub Option<specs::Entity>);
