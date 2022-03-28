pub mod assets;
pub mod systems;

use assets::*;
use clockwork::base_state::*;
use clockwork::prelude::*;
use systems::*;

use simple_logger::SimpleLogger;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColoredMeshKey {
    Monkey,
    Cone,
    Torus,
    Skybox,
}

fn main() {
    SimpleLogger::default()
        .with_utc_timestamps()
        .init()
        .unwrap();
    Clockwork::<BaseState<ColoredMeshKey>, StandardEvent>::builder()
        // MAIN LOOP
        .main_loop(main_loop)
        // STATE
        .state(
            BaseState::builder()
                .with_assets(
                    Assets::builder()
                        .materials(load_material)
                        .colored_meshes(load_colored_mesh)
                        .static_meshes(load_mesh)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        // SYSTEMS MECHANISM
        .add_mechanism(
            LegionSystems::<StandardEvent>::builder()
                .add_system(StandardEvent::Initialization, load_scene_system())
                .add_system(StandardEvent::Draw, gui_system())
                .add_system(StandardEvent::Tick, camera_control_system())
                .add_system(StandardEvent::Tick, point_light_control_system())
                .add_system(StandardEvent::Tick, load_save_system())
                .build()
                .unwrap(),
        )
        // PHYSICS MECHANISM
        .add_standard_mechanism(
            Rapier3DTicker::<MainLoopStatistics>::builder()
                .build()
                .unwrap(),
        )
        // GRAPHICS MECHANISM
        .add_standard_mechanism(
            VulkanoGraphics::builder()
                .add_layer(SkyboxDrawer::default())
                .add_layer(StaticMeshDrawer::<_, BaseState<_>, _, _, _, _, _, _, _, _>::new(0))
                .build()
                .unwrap(),
        )
        // LAUNCH
        .build()
        .unwrap()
        .set_the_clock()
}
