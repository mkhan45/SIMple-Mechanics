use rapier2d::{na::{Vector2, Isometry2}, prelude::*};

pub struct PhysicsRes {
    pub gravity: Vector2<f32>,
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: (),
    pub event_handler: (),
    pub scale_factor: f32,
}

impl PhysicsRes {
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

enum Shape {
    Rectangle(f32, f32),
    Circle(f32),
}

pub struct BodyBuilder {
    shape: Shape,
    typ: RigidBodyType,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    angular_velocity: f32,
    mass: f32,
    friction: f32,
    restitution: f32,
}

impl Default for BodyBuilder {
    fn default() -> Self {
        Self {
            shape: Shape::Circle(1.0),
            typ: RigidBodyType::Dynamic,
            position: Default::default(),
            velocity: Default::default(),
            angular_velocity: Default::default(),
            mass: Default::default(),
            friction: Default::default(),
            restitution: Default::default(),
        }
    }
}

impl BodyBuilder {
    fn build(self, physics_state: &mut PhysicsRes) -> RigidBodyHandle {
        let sf = physics_state.scale_factor;

        let rigid_body = RigidBodyBuilder::new(self.typ)
            .position(Isometry2::new(self.position * sf, 0.0))
            .linvel(self.velocity * sf)
            .angvel(self.angular_velocity)
            .build();

        let shape = match self.shape {
            Shape::Rectangle(w, h) => SharedShape::cuboid(w * sf, h * sf),
            Shape::Circle(r) => SharedShape::ball(r * sf),
        };

        let collider = ColliderBuilder::new(shape)
            .friction(self.friction)
            .restitution(self.restitution)
            .mass(self.mass)
            .build();

        let body_handle = physics_state.rigid_bodies.insert(rigid_body);
        physics_state.colliders.insert_with_parent(collider, body_handle, &mut physics_state.rigid_bodies);

        body_handle
    }
}
