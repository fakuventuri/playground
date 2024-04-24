use bevy::{prelude::*, window::PrimaryWindow};

pub struct HUDPlugin;

#[derive(Component)]
struct Crosshair;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, setup_hud)
            .add_systems(Update, crosshair_visibility);
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
