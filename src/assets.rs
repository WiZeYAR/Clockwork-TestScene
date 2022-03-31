use crate::ColoredMeshKey;
use clockwork::prelude::{components::PhongMaterial, ColoredMesh, TexturedMesh};

pub fn load_material(key: ColoredMeshKey) -> PhongMaterial {
    match key {
        ColoredMeshKey::Monkey => PhongMaterial::Textured {
            texture: clockwork::scene::fields::Texture2D::from_png_src(include_bytes!(
                "../models/texture.png"
            ))
            .unwrap(),
            specular_power: 128.0,
        },
        ColoredMeshKey::Skybox => PhongMaterial::Textured {
            texture: clockwork::scene::fields::Texture2D::from_png_src(include_bytes!(
                "../models/skybox.png"
            ))
            .unwrap(),
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
    TexturedMesh::from_obj_src(match key {
        ColoredMeshKey::Monkey => include_bytes!("../models/monkey.obj").as_slice(),
        ColoredMeshKey::Cone => include_bytes!("../models/cone.obj").as_slice(),
        ColoredMeshKey::Torus => include_bytes!("../models/torus.obj").as_slice(),
        ColoredMeshKey::Skybox => include_bytes!("../models/skybox.obj").as_slice(),
    })
    .unwrap()
}

pub fn load_colored_mesh(_: ColoredMeshKey) -> ColoredMesh {
    todo!()
}
