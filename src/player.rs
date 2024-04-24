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
    speed: f32,
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
            player: Player { speed: 500. },
            // transform: Transform::from_xyz(0., 0., 750.),
        })
        .insert(Transform::from_xyz(0., 0., 750.));
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

    // let mut velocity = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        player_transform.translation +=
            camera_transform.forward() * player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        player_transform.translation +=
            camera_transform.back() * player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.translation +=
            camera_transform.right() * player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.translation +=
            camera_transform.left() * player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Space) {
        player_transform.translation += camera_transform.up() * player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        player_transform.translation +=
            camera_transform.down() * player.speed * time.delta_seconds();
    }

    // println!("Player pos: {}", player_transform.translation);
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut rotation_move = Vec2::ZERO;
    let primary_window = window_query.single();

    match primary_window.cursor.grab_mode {
        CursorGrabMode::Locked => {}
        _ => {
            return;
        }
    }

    for ev in motion_evr.read() {
        // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        rotation_move += ev.delta;
    }

    if rotation_move.length_squared() > 0.0 {
        let mut camera_transform = camera_q.single_mut();
        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
        let window_scale = primary_window.height().min(primary_window.width());

        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        pitch -= rotation_move.y / window_scale * std::f32::consts::PI * MOUSE_SENSITIVITY;
        yaw -= rotation_move.x / window_scale * std::f32::consts::PI * MOUSE_SENSITIVITY;
        pitch = pitch.clamp(-1.54, 1.54); // 1.54 recomended // I reached aprox 1.57
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
            CursorGrabMode::Locked => {
                primary_window.cursor.grab_mode = CursorGrabMode::None;
                primary_window.cursor.visible = true;
            }
            CursorGrabMode::None => {
                primary_window.cursor.grab_mode = CursorGrabMode::Locked;
                primary_window.cursor.visible = false;
            }
            CursorGrabMode::Confined => {
                primary_window.cursor.grab_mode = CursorGrabMode::None;
                primary_window.cursor.visible = true;
            }
        }
    }
}
