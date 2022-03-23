pub mod systems {
    mod camera_control;
    mod gui;
    mod load_scene;

    pub use camera_control::*;
    pub use gui::*;
    pub use load_scene::*;
}

use clockwork::base_state::*;
use clockwork::prelude::*;

use clockwork::prelude::components::PhongMaterial;
use clockwork::scene::mesh_vertex::ColoredVertex;
use obj::{load_obj, Obj, TexturedVertex};
use std::io::BufReader;
use std::{fs::File, io::Cursor};
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
    SimpleLogger::default().init().unwrap();
    Clockwork::<BaseState<ColoredMeshKey>, StandardEvent>::builder()
        .main_loop(main_loop)
        .state(
            BaseState::builder()
                .with_assets(
                    Assets::builder()
                        .with_materials(|k| match k {
                            ColoredMeshKey::Monkey => PhongMaterial::Textured {
                                texture: {
                                    let decoder = png::Decoder::new(Cursor::new(
                                        include_bytes!("../models/texture.png").to_vec(),
                                    ));
                                    let (info, mut reader) = decoder.read_info().unwrap();
                                    let mut image_data = Vec::new();
                                    image_data.resize((info.width * info.height * 4) as usize, 0);
                                    reader.next_frame(&mut image_data).unwrap();
                                    clockwork::scene::fields::Texture2D::new(
                                        info.width as usize,
                                        info.height as usize,
                                        image_data,
                                    )
                                },
                                specular_power: 128.0,
                            },
                            ColoredMeshKey::Skybox => PhongMaterial::Textured {
                                texture: {
                                    let decoder = png::Decoder::new(Cursor::new(
                                        include_bytes!("../models/skybox.png").to_vec(),
                                    ));
                                    let (info, mut reader) = decoder.read_info().unwrap();
                                    let mut image_data = Vec::new();
                                    image_data.resize((info.width * info.height * 4) as usize, 0);
                                    reader.next_frame(&mut image_data).unwrap();
                                    clockwork::scene::fields::Texture2D::new(
                                        info.width as usize,
                                        info.height as usize,
                                        image_data,
                                    )
                                },
                                specular_power: 128.0,
                            },
                            ColoredMeshKey::Cone => PhongMaterial::Colored {
                                ambient: [255, 255, 255, 0].into(),
                                diffuse: [255, 255, 255, 0].into(),
                                specular: [255, 255, 255, 0].into(),
                                specular_power: 128.0,
                            },
                            _ => PhongMaterial::Colored {
                                ambient: [255; 3].into(),
                                diffuse: [255; 3].into(),
                                specular: [255; 3].into(),
                                specular_power: 128.0,
                            },
                        })
                        .with_colored_meshes(|k| {
                            let Obj {
                                vertices, indices, ..
                            } = load_obj(BufReader::new(
                                File::open(match k {
                                    ColoredMeshKey::Cone => "models/cone.obj",
                                    ColoredMeshKey::Monkey => "models/monkey.obj",
                                    ColoredMeshKey::Torus => "models/torus.obj",
                                    ColoredMeshKey::Skybox => "models/skybox.obj",
                                })
                                .unwrap(),
                            ))
                            .unwrap();
                            ColoredMesh {
                                indices,
                                vertices: vertices
                                    .iter()
                                    .map(
                                        |TexturedVertex {
                                             position, normal, ..
                                         }| {
                                            ColoredVertex {
                                                position: position.clone(),
                                                normal: normal.clone(),
                                                color: [0.5; 4],
                                            }
                                        },
                                    )
                                    .collect(),
                            }
                        })
                        .with_static_meshes(|k| {
                            let Obj {
                                vertices, indices, ..
                            } = load_obj(BufReader::new(
                                File::open(match k {
                                    ColoredMeshKey::Cone => "models/cone.obj",
                                    ColoredMeshKey::Monkey => "models/monkey.obj",
                                    ColoredMeshKey::Torus => "models/torus.obj",
                                    ColoredMeshKey::Skybox => "models/skybox.obj",
                                })
                                .unwrap(),
                            ))
                            .unwrap();
                            TexturedMesh {
                                indices,
                                vertices: vertices
                                    .iter()
                                    .map(
                                        |TexturedVertex {
                                             position,
                                             normal,
                                             texture,
                                         }| {
                                            clockwork::scene::mesh_vertex::TexturedVertex {
                                                position: position.clone(),
                                                normal: normal.clone(),
                                                texture_coord: [texture[0], texture[1]],
                                            }
                                        },
                                    )
                                    .collect(),
                            }
                        })
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
        // .add_standard_mechanism(
        //     VulkanoGraphics::builder()
        //         .add_layer(StaticMeshDrawer::default())
        //         .build()
        //         .unwrap(),
        // )
        .build()
        .unwrap()
        .set_the_clock()
}
