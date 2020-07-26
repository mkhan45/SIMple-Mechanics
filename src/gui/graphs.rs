use ggez::graphics::MeshBuilder;

use std::rc::Rc;

pub trait Graph {
    fn points(&self) -> Rc<Vec<[f32; 2]>>;
    fn draw(&self, builder: &mut MeshBuilder);
}

pub trait LineGraph {
    fn y_vels(&self) -> &[f32];
}

impl Graph for dyn LineGraph {
    fn points(&self) -> Rc<Vec<[f32; 2]>> {
        let y_vels = self.y_vels();
        let num_points = y_vels.len();
        let incr_width = 10.0 / num_points as f32;
        let x_vels = (0..num_points).map(|i| i as f32 * incr_width);

        let points = x_vels.zip(y_vels.iter()).map(|(x, y)| [x, *y])
            .collect::<Vec<[f32; 2]>>();

        Rc::new(points)
    }

    fn draw(&self, builder: &mut MeshBuilder) {
        builder.line(self.points().as_slice(), 0.1, ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap();
    }
}

macro_rules! create_linegraph {
    ($structname:ident) => {
        pub struct $structname {
            pub data: Vec<f32>,
        }

        impl LineGraph for $structname {
            fn y_vels(&self) -> &[f32] {
                self.data.as_slice()
            }
        }
    }
}

create_linegraph!(SpeedGraph);
create_linegraph!(RotVelGraph);
create_linegraph!(XPosGraph);
create_linegraph!(YPosGraph);
