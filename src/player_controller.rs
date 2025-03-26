use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, CursorGrabMode};

#[derive(Resource)]
pub struct MouseSettings {
    pub sensitivity: f32,
    pub speed: f32, //Eventually move this out to some player type
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00015,
            speed: 12.0,
        }
    }
}
#[derive(Resource)]
pub struct KeyBinds {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub crouch: KeyCode,
    pub grab_mouse_toggle: KeyCode,
}

impl Default for KeyBinds {
    fn default() ->Self {
        Self {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            jump: KeyCode::Space,
            crouch: KeyCode::ShiftLeft,
            grab_mouse_toggle: KeyCode::Escape,
        }
    }
}

#[derive(Component)]
pub struct Controller;

pub fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MouseSettings>,
    key_bindings: Res<KeyBinds>,
    mut query: Query<(&Controller, &mut Transform)>,
) {
    info!("Running player_move system"); // Add this to check if system runs
    if let Ok(window) = primary_window.get_single() {
        for (_player, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
            let right = Vec3::new(local_z.z, 0.0, -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        info!("Key pressed: {:?}", key); // Debugging keypresses
                        let key = *key;
                        if key == key_bindings.forward {
                            velocity += forward;
                        } else if key == key_bindings.backward {
                            velocity -= forward;
                        } else if key == key_bindings.left {
                            velocity -= right;
                        } else if key == key_bindings.right {
                            velocity += right;
                        } else if key == key_bindings.jump {
                            velocity += Vec3::Y;
                        } else if key == key_bindings.crouch {
                            velocity -= Vec3::Y;
                        }
                    }
                }
            }

            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_secs() * settings.speed;
            info!("Player Position: {:?}", transform.translation); // Debugging movement
        }
    } else {
        warn!("Primary Window not found with player_move");
    }
}


pub fn player_look(
    mouse_settings: Res<MouseSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_state: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Controller>>
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in mouse_state.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        pitch -= (mouse_settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (mouse_settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.5, 1.5); // Play around with this value

                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary Window not found with player_look");
    }
}

pub fn grab_cursor(keys: Res<ButtonInput<KeyCode>>,
                key_bindings: Res<KeyBinds>,
                mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.grab_mouse_toggle) {
            toggle_cursor_grab(&mut window);
        } else {
            warn!("Primary Window not found with grab_cursor");
        }
    }
}

pub fn toggle_cursor_grab(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false
        }
        _ => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

pub fn init_cursor_grab(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_cursor_grab(&mut window);
    } else {
         warn!("Primary Window not found with init_cursor_grab")
    }
}

