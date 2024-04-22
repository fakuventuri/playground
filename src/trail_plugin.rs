use bevy::prelude::*;

pub struct Trailplugin;

impl Plugin for Trailplugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (trail_spawn_system, trail_system));
    }
}

#[derive(Component)]
pub struct Trailed {
    timer: Timer,
    size: f32,
}

impl Trailed {
    pub fn new() -> Self {
        Trailed {
            timer: Timer::from_seconds(0.02, TimerMode::Repeating),
            size: 10.,
        }
    }
}

#[derive(Component)]
struct Trail {
    timer: Timer,
}

fn trail_spawn_system(
    // time: Res<Time>,
    mut commands: Commands,
    mut trailed_q: Query<(&mut Trailed, &Transform)>,
) {
    for (trailed, transform) in trailed_q.iter_mut() {
        // Decrease the timer
        // trailed.timer.tick(time.delta());

        // Check if it's time to spawn a new trail entity
        if true {
            // Spawn a new trail entity
            commands
                .spawn(SpriteBundle {
                    // Set up the sprite of the trail entity
                    sprite: Sprite {
                        color: Color::DARK_GRAY,
                        custom_size: Some(Vec2::splat(trailed.size)),
                        ..Default::default()
                    }, // Adjust size as needed
                    // Set up the position of the trail entity
                    transform: Transform::from_translation(transform.translation.xy().extend(-1.)), // Adjust position as needed
                    ..Default::default()
                })
                .insert(Trail {
                    timer: Timer::from_seconds(30., TimerMode::Repeating), // Adjust interval as needed
                });
        }
    }
}

fn trail_system(mut commands: Commands, time: Res<Time>, mut trail_q: Query<(Entity, &mut Trail)>) {
    for (entity, mut trail) in trail_q.iter_mut() {
        trail.timer.tick(time.delta());

        if trail.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
