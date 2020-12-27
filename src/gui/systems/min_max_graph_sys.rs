use crate::gui::graphs::*;
use specs::prelude::*;

use crate::resources::{GraphMinMax, Paused};

pub struct MinMaxGraphSys;
impl<'a> System<'a> for MinMaxGraphSys {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, SpeedGraph>,
        ReadStorage<'a, RotVelGraph>,
        ReadStorage<'a, XVelGraph>,
        ReadStorage<'a, YVelGraph>,
        ReadStorage<'a, XPosGraph>,
        ReadStorage<'a, YPosGraph>,
        ReadStorage<'a, RotGraph>,
        Read<'a, Paused>,
        Write<'a, GraphMinMax>,
    );

    fn run(
        &mut self,
        (
            speed_graphs,
            rotvel_graphs,
            xvel_graphs,
            yvel_graphs,
            xpos_graphs,
            ypos_graphs,
            rot_graphs,
            paused,
            mut min_max,
        ): Self::SystemData,
    ) {
        if paused.0 {
            return;
        }

        let (mut min, mut max) = (std::f32::INFINITY, std::f32::NEG_INFINITY);

        macro_rules! minmax_graph_storage {
            ( $graph_storage:expr ) => {
                $graph_storage.join().for_each(|graph| {
                    let (s0, s1) = graph.points();
                    s0.iter().chain(s1.iter()).for_each(|[_, v]| {
                        min = min.min(*v);
                        max = max.max(*v);
                    });
                });
            };
        }
        minmax_graph_storage!(speed_graphs);
        minmax_graph_storage!(rotvel_graphs);
        minmax_graph_storage!(xvel_graphs);
        minmax_graph_storage!(yvel_graphs);
        minmax_graph_storage!(xpos_graphs);
        minmax_graph_storage!(ypos_graphs);
        minmax_graph_storage!(rot_graphs);

        min_max.0 = min;
        min_max.1 = max;
    }
}
