use rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};

use bevy_ecs::prelude::*;

pub struct PhysicsRes {
    pub gravity: Vector2<f32>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
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

impl Default for PhysicsRes {
    fn default() -> Self {
        PhysicsRes {
            gravity: Vector2::new(0.0, 9.8),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::default(),
            island_manager: IslandManager::default(),
            broad_phase: BroadPhase::default(),
            narrow_phase: NarrowPhase::default(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
            scale_factor: 0.1,
        }
    }
}

impl PhysicsRes {
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

pub fn physics_step_sys(mut physics_res: ResMut<PhysicsRes>) {
    physics_res.step();
}

#[derive(PartialEq, Debug)]
pub enum Shape {
    Rectangle(f32, f32),
    Circle(f32),
}

pub struct BodyBuilder {
    pub shape: Shape,
    pub typ: RigidBodyType,
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub angular_velocity: f32,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
}

impl Default for BodyBuilder {
    fn default() -> Self {
        Self {
            shape: Shape::Circle(10.0),
            typ: RigidBodyType::Dynamic,
            position: Default::default(),
            velocity: Default::default(),
            angular_velocity: Default::default(),
            mass: 1.0,
            friction: Default::default(),
            restitution: Default::default(),
        }
    }
}

impl BodyBuilder {
    pub fn build(self, physics_state: &mut PhysicsRes) -> RigidBodyHandle {
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

        let body_handle = physics_state.rigid_body_set.insert(rigid_body);
        physics_state.collider_set.insert_with_parent(
            collider,
            body_handle,
            &mut physics_state.rigid_body_set,
        );

        body_handle
    }

    pub fn add_to_world(self, world: &mut World) {
        let rb = self.build(&mut world.get_resource_mut().unwrap());

        let id = world.spawn().insert(rb).id();

        world
            .get_resource_mut::<PhysicsRes>()
            .unwrap()
            .rigid_body_set
            .get_mut(rb)
            .unwrap()
            .user_data = unsafe { std::mem::transmute_copy(&id) };
    }

    pub fn add_to_world_with_commands(
        self,
        mut commands: Commands,
        mut physics_state: ResMut<PhysicsRes>,
    ) {
        let rb = self.build(&mut physics_state);
        let id = commands.spawn().insert(rb).id();

        physics_state.rigid_body_set.get_mut(rb).unwrap().user_data =
            unsafe { std::mem::transmute_copy(&id) };
    }
}
