use clockwork::prelude::{InputState, PhysicsState, VirtualKeyCode};
use legion::{any, serialize::Canon, system, systems::CommandBuffer, Registry};

use crate::systems::{COMPONENT_REGISTRY, ENTITY_SERIALIZER};

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
                serde_json::to_string(&*resources.get::<PhysicsState>().unwrap()).unwrap();
            std::fs::write("save-world.json", world).unwrap();
            std::fs::write("save-physics.json", physics).unwrap();
        });
    } else if input.pressed_keys().get(&VirtualKeyCode::F9).is_some() {
        // QUICK LOAD
    }
}
