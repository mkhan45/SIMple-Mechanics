use crate::MainState;

use specs::prelude::*;

use crate::gui::graphs::{
    RotGraph, RotVelGraph, SpeedGraph, XPosGraph, XVelGraph, YPosGraph, YVelGraph,
};
use crate::resources::{CreationData, Paused, ShapeInfo};
use crate::MechanicalWorld;

#[derive(Clone, PartialEq, Debug)]
pub enum UiSignal {
    AddShape(ShapeInfo),
    DeleteShape(Entity),
    DeleteAll,
    TogglePause,
    LoadLua(String),
    AddSpeedGraph(Entity),
    AddRotVelGraph(Entity),
    AddXVelGraph(Entity),
    AddYVelGraph(Entity),
    AddXPosGraph(Entity),
    AddYPosGraph(Entity),
    AddRotGraph(Entity),
    SerializeGraphs,
    SerializeState,
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn process_gui_signals(&mut self) {
        macro_rules! add_graph_variant {
            ( $graph_type:ident, $entity:expr ) => {
                let mut graph_storage = self.world.write_storage::<$graph_type>();
                graph_storage
                    .insert(*$entity, $graph_type::default())
                    .unwrap();
            };
        }

        self.imgui_wrapper
            .sent_signals
            .clone()
            .iter()
            .for_each(|signal| match signal {
                UiSignal::AddShape(shape_info) => {
                    self.world.insert(CreationData(Some(shape_info.clone())))
                }
                UiSignal::DeleteShape(entity) => {
                    self.delete_entity(*entity);
                    self.imgui_wrapper.remove_sidemenu(entity);
                }
                UiSignal::DeleteAll => {
                    self.delete_all();
                }
                UiSignal::TogglePause => {
                    self.world.fetch_mut::<Paused>().toggle();
                }
                UiSignal::LoadLua(filename) => {
                    self.delete_all();
                    self.add_shapes_from_lua_file(format!("lua/{}", filename));
                    self.lua_update();
                }
                //TODO: Figure out how to make macro work in top level of match, e.g.
                // add_graph_variant!(SpeedGraph) generates the whole match arm
                UiSignal::AddSpeedGraph(entity) => {
                    add_graph_variant!(SpeedGraph, entity);
                }
                UiSignal::AddRotVelGraph(entity) => {
                    add_graph_variant!(RotVelGraph, entity);
                }
                UiSignal::AddXVelGraph(entity) => {
                    add_graph_variant!(XVelGraph, entity);
                }
                UiSignal::AddYVelGraph(entity) => {
                    add_graph_variant!(YVelGraph, entity);
                }
                UiSignal::AddXPosGraph(entity) => {
                    add_graph_variant!(XPosGraph, entity);
                }
                UiSignal::AddYPosGraph(entity) => {
                    add_graph_variant!(YPosGraph, entity);
                }
                UiSignal::AddRotGraph(entity) => {
                    add_graph_variant!(RotGraph, entity);
                }
                UiSignal::SerializeGraphs => {
                    self.serialize_graphs_to_csv("out.csv");
                }
                UiSignal::SerializeState => {
                    self.export_lua("lua/test.lua");
                }
            });
        self.imgui_wrapper.sent_signals.clear();

        let lua = self.world.fetch_mut::<crate::resources::LuaRes>().clone();
        lua.lock().unwrap().context(|lua_ctx| {
            let globals = lua_ctx.globals();
            globals
                .set("GRAVITY", self.world.fetch::<MechanicalWorld>().gravity.y)
                .unwrap();
            globals
                .set("PAUSED", self.world.fetch::<Paused>().0)
                .unwrap();
        });
    }
}
