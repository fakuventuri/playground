use bevy::{app::AppExit, asset::AssetMetaCheck, prelude::*};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(ScreenMode::Windowed)
            .insert_resource(AssetMetaCheck::Never) // ??
            .add_systems(
                Update,
                (screen_mode_with_resource, set_screen_mode_with_keys),
            );

        #[cfg(debug_assertions)]
        {
            // add debug_exit_with_ctrl_w system to debug mode
            app.add_systems(Update, (bevy::window::close_on_esc, debug_exit_with_ctrl_w));

            // use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
            // app.add_plugins((
            //     FrameTimeDiagnosticsPlugin,
            //     // bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
            //     LogDiagnosticsPlugin::default(),
            // ));
        }
    }
}

// Config
// WindowMode
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum ScreenMode {
    Windowed,
    BorderlessFullscreen,
}

fn debug_exit_with_ctrl_w(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.just_pressed(KeyCode::KeyW) {
        app_exit_events.send(AppExit);
    }
}

fn set_screen_mode_with_keys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut screen_mode: ResMut<ScreenMode>,
) {
    if (keyboard_input.pressed(KeyCode::AltLeft) && keyboard_input.just_pressed(KeyCode::Enter))
        || keyboard_input.just_pressed(KeyCode::KeyF)
    {
        match *screen_mode {
            ScreenMode::Windowed => {
                *screen_mode = ScreenMode::BorderlessFullscreen;
            }
            ScreenMode::BorderlessFullscreen => {
                *screen_mode = ScreenMode::Windowed;
            }
        }
    }
}

fn screen_mode_with_resource(screen_mode: Res<ScreenMode>, mut windows: Query<&mut Window>) {
    if screen_mode.is_changed() {
        let mut window = windows.single_mut();
        match *screen_mode {
            ScreenMode::BorderlessFullscreen => {
                window.mode = bevy::window::WindowMode::BorderlessFullscreen
            }
            ScreenMode::Windowed => window.mode = bevy::window::WindowMode::Windowed,
        }
    }
}
