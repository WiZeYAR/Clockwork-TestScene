use crate::ColoredMeshKey;
use clockwork::{
    prelude::{
        components::{AmbientLight, Camera, DirectionalLight, PointLight, SpotLight},
        fields::Attenuation,
        nalgebra::{Perspective3, UnitVector3},
        Event, Isometry, PhysicsState, RigidBodyBuilder, RigidBodyType, StandardEvent, WindowEvent,
        WinitLoopProxy,
    },
    vulkano_layers::static_mesh_drawer::DrawMarker,
};
use legion::{system, systems::CommandBuffer};

#[system]
pub fn load_scene(
    world: &mut CommandBuffer,
    #[resource] PhysicsState { bodies, .. }: &mut PhysicsState,
    #[resource] main_loop: &mut WinitLoopProxy<StandardEvent>,
) {
    let ml = main_loop.clone();
    main_loop.add_event_callback(move |ev| {
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = ev
        {
            ml.trigger_event(StandardEvent::Termination)
        }
    });

    // Inserting Camera (With spot light)
    world.push((
        clockwork::vulkano_layers::skybox_drawer::DrawMarker,
        0u32,
        DrawMarker,
        SpotLight {
            color: [255, 255, 255].into(),
            attenuation: Attenuation {
                constant: 1.0,
                linear: 0.0,
                quadratic: 0.05,
            },
            opening_angle: 0.523599,
        },
        Camera::Perspective(Perspective3::new(800.0 / 600.0, 3.14 / 2.0, 0.01, 100.0)),
        bodies.insert(
            RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .position(Isometry::from_parts(
                    [0.0, 0.0, 1.0].into(),
                    Default::default(),
                ))
                .build(),
        ),
    ));

    // Inserting Skybox
    world.push((
        clockwork::vulkano_layers::skybox_drawer::DrawMarker,
        AmbientLight {
            color: [255; 3].into(),
        },
        ColoredMeshKey::Skybox,
        bodies.insert(
            RigidBodyBuilder::dynamic()
                .position(Isometry::translation(0.0, 0.0, 0.0))
                .build(),
        ),
    ));

    // Inserting Lights
    world.extend([(
        0u32,
        DrawMarker,
        DirectionalLight {
            color: [0, 128, 0].into(),
            direction: UnitVector3::new_normalize([0.0, 1.0, -1.0].into()),
        },
    )]);
    world.push((
        0u32,
        DrawMarker,
        PointLight {
            color: [128, 0, 0].into(),
            attenuation: Attenuation {
                constant: 1.0,
                linear: 0.0,
                quadratic: 0.5,
            },
        },
        bodies.insert(
            RigidBodyBuilder::dynamic()
                .position(Isometry::translation(0.0f32, 0.0, 0.0))
                .linvel([0.0, 0.5, 0.0].into())
                .build(),
        ),
    ));
    world.push((
        0u32,
        DrawMarker,
        AmbientLight {
            color: [20; 3].into(),
        },
    ));

    // Inserting Objects
    world.extend([
        (
            0u32,
            DrawMarker,
            ColoredMeshKey::Monkey,
            bodies.insert(RigidBodyBuilder::new(RigidBodyType::Dynamic).build()),
        ),
        (
            0u32,
            DrawMarker,
            ColoredMeshKey::Cone,
            bodies.insert(
                RigidBodyBuilder::new(RigidBodyType::Dynamic)
                    .position(Isometry::translation(-5f32, 0f32, 0f32))
                    .build(),
            ),
        ),
        (
            0u32,
            DrawMarker,
            ColoredMeshKey::Torus,
            bodies.insert(
                RigidBodyBuilder::new(RigidBodyType::Dynamic)
                    .position(Isometry::translation(5f32, 0f32, 0f32))
                    .angvel([1.0, 1.0, 1.0].into())
                    .build(),
            ),
        ),
    ]);
}
