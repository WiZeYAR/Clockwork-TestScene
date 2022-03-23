use clockwork::main_loop::state::InputState;
use clockwork::prelude::{
    components::{Camera, PointLight},
    nalgebra::Vector3,
    Isometry, PhysicsState, RigidBodyHandle, VirtualKeyCode,
};
use legion::system;

#[system(for_each)]
pub fn camera_control(
    _: &Camera,
    body_handle: &RigidBodyHandle,
    #[resource] input: &InputState,
    #[resource] PhysicsState { bodies, .. }: &mut PhysicsState,
) {
    let body = bodies.get_mut(body_handle.clone()).unwrap();
    let mut linvel = Default::default();
    let mut angvel = Default::default();
    let pressed_keys = input.pressed_keys();

    if pressed_keys.contains(&VirtualKeyCode::W) {
        linvel += Vector3::from([0.0, 0.0, -1.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::S) {
        linvel += Vector3::from([0.0, 0.0, 1.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::A) {
        linvel += Vector3::from([-1.0, 0.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::D) {
        linvel += Vector3::from([1.0, 0.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::Space) {
        linvel += Vector3::from([0.0, 1.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::LShift) {
        linvel += Vector3::from([0.0, -1.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::Left) {
        angvel += Vector3::from([0.0, 1.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::Right) {
        angvel += Vector3::from([0.0, -1.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::Up) {
        angvel += Vector3::from([1.0, 0.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::Down) {
        angvel += Vector3::from([-1.0, 0.0, 0.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::RShift) {
        angvel += Vector3::from([0.0, 0.0, -1.0]);
    }

    if pressed_keys.contains(&VirtualKeyCode::Slash) {
        angvel += Vector3::from([0.0, 0.0, 1.0]);
    }

    linvel = body.position() * linvel;
    angvel = body.position() * angvel;

    body.set_linvel(linvel, true);
    body.set_angvel(angvel, true);
}

#[system(for_each)]
pub fn point_light_control(
    _: &PointLight,
    body_handle: &RigidBodyHandle,
    #[resource] input: &InputState,
    #[resource] PhysicsState { bodies, .. }: &mut PhysicsState,
) {
    let body = bodies.get_mut(body_handle.clone()).unwrap();
    let pressed_keys = input.pressed_keys();

    if pressed_keys.contains(&VirtualKeyCode::Key1) {
        body.set_position(Isometry::translation(0.0, 0.0, 0.0), true);
    }
}
