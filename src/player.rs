use std::f32::consts::PI;

use bevy::{
    ecs::query::QuerySingleError,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

const MOUSE_SENSITIVITY: f32 = 0.65;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    base_speed: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // transform: Transform,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, (setup_camera, setup_player))
            .add_systems(Update, (mouse_motion, move_player, cursor_grab))
            .add_systems(Last, camera_follow_player);
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn(PlayerBundle {
            player: Player { base_speed: 500. },
            // transform: Transform::from_xyz(0., 0., 750.),
        })
        .insert(Transform::from_xyz(0., -250., 750.));
}

fn camera_follow_player(
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    match player_q.get_single() {
        Ok(Transform { translation, .. }) => {
            camera_q.single_mut().translation = *translation;
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no player!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one player!");
        }
    }
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Player)>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    let (mut player_transform, player) = player_q.single_mut();
    let camera_transform = camera_q.single();

    let mut speed = player.base_speed;
    let mut movement = Vec3::ZERO;
    let local_z = camera_transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z).normalize_or_zero();
    let right = Vec3::new(local_z.z, 0., -local_z.x).normalize_or_zero();

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += forward;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement += -forward;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        movement += right;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        movement += -right;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        // movement += camera_transform.up() * 1.;
        movement += Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        // movement += camera_transform.down() * 1.;
        movement -= Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        speed *= 2.;
    }

    player_transform.translation += movement.normalize_or_zero() * speed * time.delta_seconds();

    // println!("Player pos: {}", player_transform.translation);
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.single();

    if primary_window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let mut rotation_move = Vec2::ZERO;

    for ev in motion_evr.read() {
        // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        rotation_move += ev.delta;
    }

    if rotation_move.length_squared() > 0.0 {
        let mut camera_transform = camera_q.single_mut();
        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
        let window_scale = primary_window.height().min(primary_window.width());

        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= rotation_move.x / window_scale * PI * MOUSE_SENSITIVITY;
        pitch -= rotation_move.y / window_scale * PI * MOUSE_SENSITIVITY;

        pitch = pitch.clamp(-1.56, 1.56); // PI / 2.

        camera_transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        let mut primary_window = q_windows.single_mut();

        match primary_window.cursor.grab_mode {
            CursorGrabMode::None => {
                primary_window.cursor.grab_mode = CursorGrabMode::Locked;
                primary_window.cursor.visible = false;
            }
            CursorGrabMode::Locked => {
                primary_window.cursor.grab_mode = CursorGrabMode::None;
                primary_window.cursor.visible = true;
            }
            CursorGrabMode::Confined => {
                primary_window.cursor.grab_mode = CursorGrabMode::None;
                primary_window.cursor.visible = true;
            }
        }
    }
}
