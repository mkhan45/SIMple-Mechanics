use ncollide2d as nc;
use nphysics2d as np;

pub type Vector = nalgebra::Vector2<f32>;
pub type Point = nalgebra::Point2<f32>;

pub type MechanicalWorld = np::world::DefaultMechanicalWorld<f32>;
pub type BodySet = np::object::DefaultBodySet<f32>;
pub type GeometricalWorld = np::world::DefaultGeometricalWorld<f32>;
pub type ColliderSet = np::object::DefaultColliderSet<f32>;
pub type JointConstraintSet = np::joint::DefaultJointConstraintSet<f32>;
pub type ForceGeneratorSet = np::force_generator::DefaultForceGeneratorSet<f32>;

pub type ShapeHandle = nc::shape::ShapeHandle<f32>;
pub type ColliderHandle = np::object::DefaultColliderHandle;
pub type RigidBody = np::object::RigidBody<f32>;
pub type RigidBodyDesc = np::object::RigidBodyDesc<f32>;
