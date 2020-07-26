use ggez::graphics::MeshBuilder;

pub trait Graph {
    fn draw(&self, builder: &mut MeshBuilder);
}

pub trait LineGraph {
    fn add_val(&mut self, val: f32);
    fn points(&self) -> &[[f32; 2]];
}

impl Graph for dyn LineGraph {
    fn draw(&self, builder: &mut MeshBuilder) {
        builder.line(self.points(), 0.1, ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap();
    }
}

macro_rules! create_linegraph {
    ($structname:ident) => {
        pub struct $structname {
            pub data: Vec<[f32; 2]>,
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
        }
    }
}

create_linegraph!(SpeedGraph);
create_linegraph!(RotVelGraph);
create_linegraph!(XPosGraph);
create_linegraph!(YPosGraph);
