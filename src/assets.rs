use std::{
    fs::File,
    io::{BufReader, Cursor},
};

use clockwork::{
    prelude::{components::PhongMaterial, ColoredMesh, TexturedMesh},
    scene::mesh_vertex::ColoredVertex,
};
use obj::{load_obj, Obj, TexturedVertex};

use crate::ColoredMeshKey;

pub fn load_material(key: ColoredMeshKey) -> PhongMaterial {
    match key {
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
                let decoder =
                    png::Decoder::new(Cursor::new(include_bytes!("../models/skybox.png").to_vec()));
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
    }
}

pub fn load_mesh(key: ColoredMeshKey) -> TexturedMesh {
    let Obj {
        vertices, indices, ..
    } = load_obj(BufReader::new(
        File::open(match key {
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
}

pub fn load_colored_mesh(key: ColoredMeshKey) -> ColoredMesh {
    let Obj {
        vertices, indices, ..
    } = load_obj(BufReader::new(
        File::open(match key {
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
}
