mod camera_control;
mod gui;
mod load_save;
mod load_scene;

pub use camera_control::*;
use clockwork::{
    prelude::{
        components::{AmbientLight, Camera, DirectionalLight, PointLight, SpotLight},
        RigidBodyHandle,
    },
    vulkano_layers::{skybox_drawer::DrawMarker, static_mesh_drawer::DrawMarker as DM},
};
pub use gui::*;
pub use load_save::*;
pub use load_scene::*;

use crate::ColoredMeshKey;

macro_rules! mkregistry {
    ($($data_type:ty)+) => {{
        let mut registry = legion::Registry::default();
        $(registry.register::<$data_type>(stringify!($data_type).into()));+;
        registry
    }};
}

lazy_static::lazy_static! {
    static ref ENTITY_SERIALIZER: legion::serialize::Canon = Default::default();
    static ref COMPONENT_REGISTRY: legion::Registry<String> = mkregistry!(
        i8 i16 i32 i64 i128 isize
        u8 u16 u32 u64 u128 usize
        f32 f64
        bool char
        String

        AmbientLight DirectionalLight PointLight SpotLight
        DrawMarker DM
        Camera RigidBodyHandle ColoredMeshKey
    );
}
