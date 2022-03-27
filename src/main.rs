pub mod systems {
    mod camera_control;
    mod gui;
    mod load_scene;

    pub use camera_control::*;
    pub use gui::*;
    pub use load_scene::*;
}
pub mod assets;

use assets::*;
use clockwork::base_state::*;
use clockwork::prelude::*;
use systems::*;

use simple_logger::SimpleLogger;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
        .main_loop(main_loop)
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
        .add_mechanism(
            LegionSystems::<StandardEvent>::builder()
                .add_system(StandardEvent::Initialization, load_scene_system())
                .add_system(StandardEvent::Draw, gui_system())
                .add_system(StandardEvent::Tick, camera_control_system())
                .add_system(StandardEvent::Tick, point_light_control_system())
                .build()
                .unwrap(),
        )
        .add_standard_mechanism(
            Rapier3DTicker::<MainLoopStatistics>::builder()
                .build()
                .unwrap(),
        )
        .add_standard_mechanism(
            VulkanoGraphics::builder()
                .add_layer(SkyboxDrawer::default())
                .add_layer(StaticMeshDrawer::<_, BaseState<_>, _, _, _, _, _, _, _, _>::new(0))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
        .set_the_clock()
}
