use ggez::graphics::MeshBuilder;
use specs::storage::BTreeStorage;
use specs::Component;

use csv;

pub trait Graph {
    fn draw(&self, builder: &mut MeshBuilder);
    fn serialize_csv(&self);
}

pub trait LineGraph {
    fn add_val(&mut self, val: f32);
    fn points(&self) -> &[[f32; 2]];
    fn name(&self) -> String;
}

impl Graph for dyn LineGraph {
    fn draw(&self, builder: &mut MeshBuilder) {
        builder
            .line(
                self.points(),
                0.1,
                ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            )
            .unwrap();
    }

    fn serialize_csv(&self) {
        let mut writer = csv::Writer::from_writer(std::io::stdout());

        writer.write_record(&[self.name()]).unwrap();
        self.points().iter().for_each(|[_, val]| {
            writer.write_record(&[val.to_string()]).unwrap();
        });
        writer.flush().unwrap();
    }
}

macro_rules! create_linegraph {
    ($structname:ident, $name:expr) => {
        #[derive(Debug, Clone, Component)]
        #[storage(BTreeStorage)]
        pub struct $structname {
            pub data: Vec<[f32; 2]>,
        }

        impl Default for $structname {
            fn default() -> Self {
                $structname {
                    data: Vec::with_capacity(100),
                }
            }
        }

        impl LineGraph for $structname {
            fn points(&self) -> &[[f32; 2]] {
                self.data.as_slice()
            }

            fn add_val(&mut self, val: f32) {
                let num_vals = self.data.len() + 1;
                let step_incr = 10.0 / num_vals as f32;
                self.data.iter_mut().enumerate().for_each(|(i, [_, y])| {
                    *y = step_incr * i as f32;
                });
                self.data.push([val, 10.0]);
            }

            fn name(&self) -> String {
                $name.to_string()
            }
        }
    };
}

create_linegraph!(SpeedGraph, "Speed");
create_linegraph!(RotVelGraph, "Rotational Velocity");
create_linegraph!(XPosGraph, "X Position");
create_linegraph!(YPosGraph, "Y Position");
