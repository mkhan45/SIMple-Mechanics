use crate::{Point, Vector};
use ggez::graphics::{self, Rect};

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
// frame time
pub struct DT(pub std::time::Duration);

#[derive(Copy, Clone)]
// physics step in seconds
pub struct Timestep(pub f32);
impl Default for Timestep {
    fn default() -> Self {
        Timestep(0.016)
    }
}

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

#[derive(Clone, Default)]
pub struct SaveSceneFilename(pub String);

#[derive(Clone, Default)]
pub struct SaveGraphFilename(pub String);

#[derive(Copy, Clone)]
pub struct ScaleFac(pub Vector);
impl Default for ScaleFac {
    fn default() -> Self {
        ScaleFac(Vector::new(1.0, 1.0))
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    pub pos: Vector,
    pub scale: f32,
}

impl Camera {
    pub fn make_drawparam(&self) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .dest([self.pos.x, self.pos.y])
            .scale([self.scale, self.scale])
    }

    pub fn change_scale(&mut self, delta: f32, focus: Vector) {
        let prev_scale = self.scale;
        let new_scale = (self.scale + delta).max(0.05);

        let delta_focus = {
            let prev_scaled_focus = focus;
            let new_scaled_focus = focus / new_scale * prev_scale;
            (new_scaled_focus - prev_scaled_focus) * prev_scale
        };

        self.pos += delta_focus;
        self.scale = new_scale;
    }

    pub fn translate(&mut self, vector: Vector) {
        self.pos += -vector * self.scale;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Vector::new(0.0, 0.0),
            scale: 1.0,
        }
    }
}
