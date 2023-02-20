use bevy::{input::mouse::MouseMotion, prelude::*};

const CAMERA_MOVEMENT_SPEED: f32 = 0.25;
const CAMERA_ROTATION_SPEED: f32 = 0.005;

#[derive(Default, Resource)]
pub struct CameraState {
    pub direction: Vec2,
}

pub fn move_camera(
    mut camera_state: ResMut<CameraState>,
    mut transforms: Query<&mut Transform, With<Camera3d>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    for mut transform in &mut transforms {
        if keyboard_input.pressed(KeyCode::W) {
            let forward = transform.forward();
            transform.translation += forward * CAMERA_MOVEMENT_SPEED;
        } else if keyboard_input.pressed(KeyCode::S) {
            let back = transform.back();
            transform.translation += back * CAMERA_MOVEMENT_SPEED;
        } else if keyboard_input.pressed(KeyCode::A) {
            let left = transform.left();
            transform.translation += left * CAMERA_MOVEMENT_SPEED;
        } else if keyboard_input.pressed(KeyCode::D) {
            let right = transform.right();
            transform.translation += right * CAMERA_MOVEMENT_SPEED;
        }
    }

    for event in mouse_motion_events.iter() {
        let delta = event.delta;

        camera_state.direction += delta * CAMERA_ROTATION_SPEED;
        camera_state.direction.x %= 2.0 * std::f32::consts::PI;
        camera_state.direction.y = camera_state
            .direction
            .y
            .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

        for mut transform in &mut transforms {
            transform.rotation = Quat::from_euler(
                EulerRot::YXZ,
                -camera_state.direction.x,
                -camera_state.direction.y,
                0.0,
            )
        }
    }
}
