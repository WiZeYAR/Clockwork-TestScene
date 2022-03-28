use clockwork::prelude::{InputState, PhysicsState, VirtualKeyCode};
use legion::{any, serialize::Canon, system, systems::CommandBuffer, Registry};
use serde::de::DeserializeSeed;

use crate::systems::{COMPONENT_REGISTRY, ENTITY_SERIALIZER};

static SAVE_WORLD: &'static str = "save-world.json";
static SAVE_PHYSICS: &'static str = "save-physics.json";

#[system]
pub fn load_save(cmd: &mut CommandBuffer, #[resource] input: &InputState) {
    if input.pressed_keys().get(&VirtualKeyCode::F5).is_some() {
        // QUICK SAVE
        cmd.exec_mut(|world, resources| {
            let world_serializer: &Registry<_> = &COMPONENT_REGISTRY;
            let entity_serializer: &Canon = &ENTITY_SERIALIZER;
            let world = serde_json::to_string(&world.as_serializable(
                any(),
                world_serializer,
                entity_serializer,
            ))
            .unwrap();
            let physics =
                serde_json::to_string_pretty(&*resources.get::<PhysicsState>().unwrap()).unwrap();
            std::fs::write(SAVE_WORLD, world).unwrap();
            std::fs::write(SAVE_PHYSICS, physics).unwrap();
        });
    } else if input.pressed_keys().get(&VirtualKeyCode::F9).is_some() {
        // QUICK LOAD
        cmd.exec_mut(|world, resources| {
            let world_serializer: &Registry<_> = &COMPONENT_REGISTRY;
            let entity_serializer: &Canon = &ENTITY_SERIALIZER;
            *world = std::fs::read_to_string(SAVE_WORLD)
                .map(|x| serde_json::from_str::<serde_json::Value>(&x).unwrap())
                .map(|x| {
                    world_serializer
                        .as_deserialize(entity_serializer)
                        .deserialize(&x)
                        .unwrap()
                })
                .unwrap();
            std::fs::read_to_string(SAVE_PHYSICS)
                .map(|file| serde_json::from_str::<PhysicsState>(&file).unwrap())
                .map(|saved_state| {
                    let resource: &mut PhysicsState =
                        &mut resources.get_mut::<PhysicsState>().unwrap();
                    *resource = saved_state;
                })
                .unwrap();
        });
    }
}
