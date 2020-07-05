use smallvec::SmallVec;

use ggez::event::EventHandler;
use ggez::graphics;

use specs::prelude::*;

use crate::{
    BodySet, Collider, ColliderSet, ForceGeneratorSet, GeometricalWorld, JointConstraintSet,
    MechanicalWorld, Point, ShapeHandle, Vector,
};
use crate::{SCREEN_X, SCREEN_Y};

use crate::components::*;

use crate::lua::LuaResExt;
use crate::resources::{self, LuaRes, ShapeInfo};
use crate::RigidBodyDesc;

use ncollide2d as nc;
use nphysics2d as np;

pub struct MainState<'a, 'b> {
    pub world: specs::World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

pub struct BodyBuilder<'a> {
    body_set: Write<'a, BodySet>,
    collider_set: Write<'a, ColliderSet>,
    lazy_update: Read<'a, LazyUpdate>,
    entities: Entities<'a>,
    shape: ShapeHandle,
    mass: f32,
    translation: Vector,
    rotation: f32,
    velocity: Vector,
    rotvel: f32,
    status: np::object::BodyStatus,
    restitution: f32,
    friction: f32,
    color: ggez::graphics::Color,
    name: Option<String>,
}

impl<'a> BodyBuilder<'a> {
    pub fn new(
        body_set: Write<'a, BodySet>,
        collider_set: Write<'a, ColliderSet>,
        lazy_update: Read<'a, LazyUpdate>,
        entities: Entities<'a>,
        shape_info: ShapeInfo,
        mass: f32,
    ) -> Self {
        let shape = match shape_info {
            ShapeInfo::Circle(r) => ShapeHandle::new(nc::shape::Ball::new(r)),
            ShapeInfo::Rectangle(v) => ShapeHandle::new(nc::shape::Cuboid::new(v)),
        };

        BodyBuilder {
            body_set,
            collider_set,
            lazy_update,
            entities,
            shape,
            mass,
            translation: Vector::new(0.0, 0.0),
            rotation: 0.0,
            velocity: Vector::new(0.0, 0.0),
            rotvel: 0.0,
            status: np::object::BodyStatus::Dynamic,
            restitution: 0.2,
            friction: 0.5,
            color: ggez::graphics::WHITE,
            name: None,
        }
    }

    pub fn create(mut self) -> Entity {
        let body = RigidBodyDesc::new()
            .mass(self.mass)
            .translation(self.translation)
            .rotation(self.rotation)
            .velocity(np::math::Velocity::new(self.velocity, self.rotvel))
            .status(self.status)
            .build();

        let body_handle = self.body_set.insert(body);

        let coll = np::object::ColliderDesc::new(self.shape)
            .density(1.0)
            .set_material(np::material::MaterialHandle::new(
                np::material::BasicMaterial::new(self.restitution, self.friction),
            ))
            .build(np::object::BodyPartHandle(body_handle, 0));

        let coll_handle = self.collider_set.insert(coll);

        let mut specs_handle = self
            .lazy_update
            .create_entity(&self.entities)
            .with(PhysicsBody { body_handle })
            .with(Collider { coll_handle })
            .with(Color(self.color));

        if let Some(n) = self.name {
            specs_handle = specs_handle.with(Name(n));
        }

        let specs_handle = specs_handle.entity;

        self.body_set
            .rigid_body_mut(body_handle)
            .unwrap()
            .set_user_data(Some(Box::new(specs_handle)));

        self.collider_set
            .get_mut(coll_handle)
            .unwrap()
            .set_user_data(Some(Box::new(specs_handle)));

        specs_handle
    }
}

impl<'a, 'b> MainState<'a, 'b> {
    #[allow(clippy::many_single_char_names)]
    /// must call world.maintain() after calling this for shape to actually get added
    /// in practice this should only be used in process_lua_shapes() so it should be fine
    pub fn process_lua_shape(&mut self, shape: &rlua::Table) {
        let ty: String = shape.get("shape").unwrap();
        let mass = shape.get("mass").unwrap_or(1.0);
        let x = shape.get("x").unwrap();
        let y = shape.get("y").unwrap();
        let x_vel = shape.get("x_vel").unwrap_or(0.0);
        let y_vel = shape.get("y_vel").unwrap_or(0.0);
        let rotvel = shape.get("rotvel").unwrap_or(0.0);
        let rotation = shape.get("rotation").unwrap_or(0.0);
        let elasticity = shape.get("elasticity").unwrap_or(0.2);
        let friction = shape.get("friction").unwrap_or(0.5);
        let name = shape.get("name");
        let status = shape
            .get("status")
            .unwrap_or_else(|_| "dynamic".to_string());
        let color = shape
            .get("color")
            .map_or(ggez::graphics::WHITE, |color: rlua::Table| {
                let r = color.get("r").unwrap();
                let g = color.get("g").unwrap();
                let b = color.get("b").unwrap();
                let a = color.get("a").unwrap_or(255);
                ggez::graphics::Color::from_rgba(r, g, b, a)
            });

        #[allow(clippy::wildcard_in_or_patterns)]
        let status = match status.to_lowercase().as_str() {
            "static" => np::object::BodyStatus::Static,
            "kinematic" => np::object::BodyStatus::Kinematic,
            "dynamic" | _ => np::object::BodyStatus::Dynamic,
        };

        let shape_info = match ty.to_lowercase().as_str() {
            "rectangle" | "rect" => {
                let w = shape.get("w").unwrap();
                let h = shape.get("h").unwrap();
                ShapeInfo::Rectangle(Vector::new(w, h))
            }
            "circle" => {
                let rad = shape.get("r").unwrap();
                ShapeInfo::Circle(rad)
            }
            _ => panic!("invalid shape"),
        };

        BodyBuilder {
            translation: Vector::new(x, y),
            rotation,
            velocity: Vector::new(x_vel, y_vel),
            rotvel,
            status,
            restitution: elasticity,
            friction,
            color,
            name: name.ok(),
            ..BodyBuilder::new(
                self.world.fetch_mut::<BodySet>().into(),
                self.world.fetch_mut::<ColliderSet>().into(),
                self.world.fetch::<LazyUpdate>().into(),
                self.world.entities(),
                shape_info,
                mass,
            )
        }
        .create();
    }

    pub fn process_lua_shapes(&mut self, shapes: Vec<rlua::Table>) {
        shapes
            .iter()
            .for_each(|shape| self.process_lua_shape(shape));
        self.world.maintain();
    }

    pub fn add_shapes_from_lua_file(
        &mut self,
        filename: impl AsRef<std::path::Path> + std::clone::Clone,
    ) {
        let lua = self.world.fetch_mut::<LuaRes>().clone();
        lua.run_lua_file(filename);
        lua.lock().unwrap().context(|lua_ctx| {
            let globals = lua_ctx.globals();
            let shapes = globals.get::<_, Vec<rlua::Table>>("shapes").unwrap();
            self.process_lua_shapes(shapes);

            let shapes: Vec<rlua::Table> = Vec::new();
            globals.set("shapes", shapes).unwrap();
        });
    }

    pub fn lua_update(&mut self) {
        let lua = self.world.fetch_mut::<crate::resources::LuaRes>().clone();
        lua.lock().unwrap().context(|lua_ctx| {
            lua_ctx.load("update()").exec().unwrap();
            let globals = lua_ctx.globals();
            if let Ok(true) = globals.get("ADD_SHAPES") {
                self.process_lua_shapes(globals.get::<_, Vec<rlua::Table>>("shapes").unwrap());
            }

            globals.set("ADD_SHAPES", false).unwrap();
            globals
                .set("FPS", self.world.fetch::<resources::FPS>().0)
                .unwrap();
            globals
                .set("DT", self.world.fetch::<resources::DT>().0.as_millis())
                .unwrap();

            {
                pub struct LuaEntity(pub Entity);
                impl rlua::UserData for LuaEntity {
                    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                        methods.add_method("id", |_, this, _: ()| Ok(this.0.id()));
                    }
                }

                let entities = self.world.entities();
                let names = self.world.read_storage::<Name>();
                let lua_objects = lua_ctx.create_table().unwrap();
                (&entities, &names).join().for_each(|(entity, name)| {
                    lua_objects.set(name.0.as_str(), LuaEntity(entity)).unwrap();
                });
                globals.set("OBJECTS", lua_objects).unwrap();
            }

            {
                let mouse_pos = self.world.fetch::<resources::MousePos>().0;
                globals.set("MOUSE_X", mouse_pos.x).unwrap();
                globals.set("MOUSE_Y", mouse_pos.y).unwrap();
            }

            let shapes: Vec<rlua::Table> = Vec::new();
            globals.set("shapes", shapes).unwrap();
        });
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        {
            self.world.insert(resources::DT(ggez::timer::delta(ctx)));
            self.world.insert(resources::FPS(ggez::timer::fps(ctx)));
        }

        {
            self.lua_update();
        }

        {
            self.dispatcher.dispatch(&self.world);
        }

        let geometrical_world = &mut self.world.fetch_mut::<GeometricalWorld>();
        let body_set = &mut *self.world.fetch_mut::<BodySet>();
        let collider_set = &mut *self.world.fetch_mut::<ColliderSet>();
        let joint_constraint_set = &mut *self.world.fetch_mut::<JointConstraintSet>();
        let force_generator_set = &mut *self.world.fetch_mut::<ForceGeneratorSet>();

        self.world.fetch_mut::<MechanicalWorld>().step(
            geometrical_world,
            body_set,
            collider_set,
            joint_constraint_set,
            force_generator_set,
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use graphics::Color;

        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut mesh_builder = graphics::MeshBuilder::new();

        let entities = self.world.entities();

        let colliders = self.world.read_storage::<Collider>();
        let colors = self.world.read_storage::<crate::components::Color>();
        let collider_set = self.world.fetch::<ColliderSet>();

        let selected = self.world.read_storage::<Selected>();

        (&colliders, &colors, &entities)
            .join()
            .for_each(|(collider_comp, color, e)| {
                let collider = collider_set
                    .get(collider_comp.coll_handle)
                    .expect("error getting body to draw");

                let (pos, rot) = {
                    let isometry = collider.position();
                    let na_vector = isometry.translation.vector;
                    ([na_vector.x, na_vector.y], isometry.rotation.angle())
                };

                if collider.shape().is_shape::<nc::shape::Ball<f32>>() {
                    let shape = collider
                        .shape()
                        .downcast_ref::<nc::shape::Ball<f32>>()
                        .expect("bad shape");

                    draw_circle(&mut mesh_builder, pos, rot, shape.radius(), color.0, false);

                    if selected.get(e).is_some() {
                        draw_circle(
                            &mut mesh_builder,
                            pos,
                            rot,
                            shape.radius(),
                            Color::new(1.0, 0.0, 0.0, 1.0),
                            true,
                        );
                    }
                } else if collider.shape().is_shape::<nc::shape::Cuboid<f32>>() {
                    let shape = collider
                        .shape()
                        .downcast_ref::<nc::shape::Cuboid<f32>>()
                        .expect("bad shape");

                    draw_rect(
                        &mut mesh_builder,
                        pos,
                        rot,
                        *shape.half_extents(),
                        color.0,
                        false,
                    );

                    if selected.get(e).is_some() {
                        draw_rect(
                            &mut mesh_builder,
                            pos,
                            rot,
                            *shape.half_extents(),
                            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                            true,
                        );
                    }
                }
            });

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) {
        let aspect_ratio = height / width;
        let initial_ratio = 1.0;

        if initial_ratio > aspect_ratio {
            let new_width = SCREEN_X / aspect_ratio;
            ggez::graphics::set_screen_coordinates(
                ctx,
                ggez::graphics::Rect::new(0.0, 0.0, new_width, SCREEN_Y),
            )
            .expect("error resizing");
        } else {
            let new_height = SCREEN_Y * aspect_ratio;
            ggez::graphics::set_screen_coordinates(
                ctx,
                ggez::graphics::Rect::new(0.0, 0.0, SCREEN_X, new_height),
            )
            .expect("error resizing");
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let screen_size = graphics::drawable_size(ctx);
        let screen_coords = graphics::screen_coordinates(ctx);
        let mouse_point = Vector::new(
            x / screen_size.0 * screen_coords.w,
            y / screen_size.1 * screen_coords.h,
        );

        self.world.insert(resources::MousePos(mouse_point));
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        btn: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let ggez::input::mouse::MouseButton::Left = btn {
            let geometrical_world = self.world.fetch::<GeometricalWorld>();
            let colliders = self.world.fetch::<ColliderSet>();

            let mouse_point = self.world.fetch::<resources::MousePos>().0;
            println!("{} {}", mouse_point.x, mouse_point.y);

            let mut selected = self.world.write_storage::<Selected>();

            geometrical_world
                .interferences_with_point(
                    &*colliders,
                    &Point::new(mouse_point.x, mouse_point.y),
                    &nc::pipeline::CollisionGroups::new(),
                )
                .for_each(|obj| {
                    let specs_hand = obj.1.user_data().unwrap();
                    let ent = specs_hand.downcast_ref::<Entity>().unwrap();

                    selected.insert(*ent, Selected::default()).unwrap();
                });
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        btn: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let ggez::input::mouse::MouseButton::Left = btn {
            let mut selected = self.world.write_storage::<Selected>();
            selected.clear();
        }
    }
}

fn draw_circle(
    mesh_builder: &mut ggez::graphics::MeshBuilder,
    pos: [f32; 2],
    rot: f32,
    rad: f32,
    color: graphics::Color,
    outline: bool,
) {
    let drawmode = if outline {
        graphics::DrawMode::stroke(rad * 0.05)
    } else {
        graphics::DrawMode::fill()
    };

    mesh_builder.circle(drawmode, pos, rad, 0.01, color);

    mesh_builder.circle(
        drawmode,
        [
            pos[0] + rad * rot.cos() * 0.75,
            pos[1] + rad * rot.sin() * 0.75,
        ],
        rad * 0.15,
        0.01,
        graphics::Color::new(0.0, 0.0, 0.0, 1.0),
    );
}

fn draw_rect(
    mesh_builder: &mut ggez::graphics::MeshBuilder,
    center_pos: [f32; 2],
    rot: f32,
    half_extents: Vector,
    color: graphics::Color,
    outline: bool,
) {
    let rot_cos = rot.cos();
    let rot_sin = rot.sin();

    // rect points in clockwise (important for ggez)
    let _points = [
        Point::new(
            center_pos[0] - half_extents.x,
            center_pos[1] - half_extents.y,
        ),
        Point::new(
            center_pos[0] + half_extents.x,
            center_pos[1] - half_extents.y,
        ),
        Point::new(
            center_pos[0] + half_extents.x,
            center_pos[1] + half_extents.y,
        ),
        Point::new(
            center_pos[0] - half_extents.x,
            center_pos[1] + half_extents.y,
        ),
    ]
    .iter()
    .map(|point| {
        // new x position is cos(theta) * (p.x - c.x) - sin(theta) * (p.y - c.y) + c.x
        // new y position is sin(theta) * (p.x - c.x) + cos(theta) * (p.y - c.y) + c.y
        [
            rot_cos * (point.x - center_pos[0]) - rot_sin * (point.y - center_pos[1])
                + center_pos[0],
            rot_sin * (point.x - center_pos[0])
                + rot_cos * (point.y - center_pos[1])
                + center_pos[1],
        ]
    })
    .collect::<SmallVec<[[f32; 2]; 4]>>();

    let points = _points.as_slice();

    let drawmode = if outline {
        graphics::DrawMode::stroke(half_extents.x.min(half_extents.y) * 0.125)
    } else {
        graphics::DrawMode::fill()
    };

    mesh_builder
        .polygon(drawmode, points, color)
        .expect("error drawing rotated rect");
}
