use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};

use crate::GameSpeed;

pub struct HUDPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct Crosshair;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_hud)
            .add_systems(
                Update,
                (
                    crosshair_visibility,
                    fps_text_update_system,
                    speed_text_update_system,
                ),
            );
    }
}

fn setup_hud(mut commands: Commands) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Crosshair
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(8.),
                        height: Val::Px(8.),
                        position_type: PositionType::Absolute,
                        // align_items: AlignItems::Center,
                        // justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .insert(Crosshair);
        });

    // // Speed text
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             // width: Val::Auto,
    //             // height: Val::Percent(100.0),
    //             position_type: PositionType::Absolute,
    //             bottom: Val::Px(10.),
    //             left: Val::Px(10.),
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::Center,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         // Crosshair
    //         parent.spawn(NodeBundle {
    //             style: Style {
    //                 width: Val::Px(8.),
    //                 height: Val::Px(8.),
    //                 position_type: PositionType::Absolute,
    //                 ..default()
    //             },
    //             background_color: Color::rgb(1.0, 1.0, 1.0).into(),
    //             ..default()
    //         });
    //     });

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 25.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 25.0,
                color: Color::GREEN,
                ..default()
            }),
        ]),
        FpsText,
    ));

    // SpeedText
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "SPEED: ",
                TextStyle {
                    font_size: 25.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 25.0,
                color: Color::GOLD,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        SpeedText,
    ));
}

fn crosshair_visibility(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut crosshair_q: Query<&mut Visibility, With<Crosshair>>,
) {
    let primary_window = window_query.single();
    let mut crosshair_visibility = crosshair_q.single_mut();

    match primary_window.cursor.grab_mode {
        bevy::window::CursorGrabMode::Locked => {
            *crosshair_visibility = Visibility::Visible;
        }
        _ => {
            *crosshair_visibility = Visibility::Hidden;
        }
    }
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn speed_text_update_system(
    mut query: Query<&mut Text, With<SpeedText>>,
    game_speed: Res<GameSpeed>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("x{:.2}", game_speed.speed);
    }
}
