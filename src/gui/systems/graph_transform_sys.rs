use crate::resources::{GraphPosData, MousePos, MovingGraph, ScalingGraph};
use crate::Vector;
use specs::prelude::*;

pub struct GraphTransformSys;
impl<'a> System<'a> for GraphTransformSys {
    type SystemData = (
        Read<'a, MousePos>,
        Write<'a, GraphPosData>,
        Read<'a, MovingGraph>,
        Read<'a, ScalingGraph>,
    );

    fn run(
        &mut self,
        (mouse_pos, mut graph_pos_data, moving_graph, scaling_graph): Self::SystemData,
    ) {
        if moving_graph.0 {
            graph_pos_data.0.x = mouse_pos.0.x - graph_pos_data.0.w;
            graph_pos_data.0.y = mouse_pos.0.y - graph_pos_data.0.h;
        }

        if scaling_graph.0 {
            let scale_mag = {
                let graph_pos_vec = Vector::new(graph_pos_data.0.x, graph_pos_data.0.y);
                (mouse_pos.0 - graph_pos_vec).norm()
            };

            // sqrt(200) is the diagonal length of the drawing area of the graph meshbuilder
            let scale_fac = scale_mag / (200.0f32).sqrt();
            let side_len = scale_fac * 10.0;

            graph_pos_data.0.w = side_len;
            graph_pos_data.0.h = side_len;
        }
    }
}
