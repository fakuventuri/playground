mod trail_plugin;
mod utils;

use std::f32::consts::PI;

use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use trail_plugin::{Trailed, Trailplugin};
use utils::*;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

const SCALE: f32 = 8. / 1000.;
const DISTANCE_SHORTENER: f32 = 10.; // for better display
const TIME_SPEED: f32 = 233280.; // moon orbit 27 days = 2332800s / 10 for 10 sec rotation

fn main() {
    App::new() //
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),
        //     FrameTimeDiagnosticsPlugin::default(),
        //     bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        // )) // Debug Diagnostics
        .add_plugins(Trailplugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (calculate_velocity, move_planets).chain())
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PlanetBundle::new(
        &mut meshes,
        &mut materials,
        6378.,
        5.51,
        Color::BLUE,
        Transform::from_xyz(0.0, 0.0, 0.0),
        None,
    ));

    commands
        .spawn(PlanetBundle::new(
            &mut meshes,
            &mut materials,
            1737.4,
            3.34,
            Color::WHITE,
            Transform::from_xyz(0., 384400. * SCALE / DISTANCE_SHORTENER, 0.0),
            Some(Vec2::new(1., 0.) * 1022. / 2332800. * 2.),
        ))
        .insert(Trailed::new());

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     &mut materials,
    //     3000.,
    //     10.,
    //     Color::RED,
    //     Transform::from_xyz(-275.0, 13.0, 0.0),
    //     None,
    // ));

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     &mut materials,
    //     3000.,
    //     10.,
    //     Color::GREEN,
    //     Transform::from_xyz(175.0, -13.0, 0.0),
    //     None,
    // ));

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     &mut materials,
    //     3000.,
    //     10.,
    //     Color::BLUE,
    //     Transform::from_xyz(-175.0, -130.0, 0.0),
    //     None,
    // ));
}

fn calculate_velocity(
    time: Res<Time>,
    mut planets_query: Query<(Entity, &Transform, &mut Planet)>,
) {
    let mut changes: Vec<Vec2> = Vec::new();

    for (entity1, transform1, planet1) in &planets_query {
        let mut calculated_velocity = Vec2::ZERO;

        for (entity2, transform2, planet2) in &planets_query {
            if !entity1.eq(&entity2) {
                let direction_to_entity2 = (transform2.translation - transform1.translation)
                    .truncate()
                    .normalize_or_zero();

                let mass1 = ((4. / 3.) * PI * planet1.radius.powi(3)) * planet1.density;
                let mass2 = ((4. / 3.) * PI * planet2.radius.powi(3)) * planet2.density;

                let distance = transform1.translation.distance(transform2.translation);

                let mut force =
                    calculate_force(mass1, mass2, distance / SCALE * DISTANCE_SHORTENER);

                if distance < (planet1.radius + planet2.radius) * SCALE {
                    force = force * (distance / (planet1.radius + planet2.radius) * SCALE);
                }

                // println!("dist: {}", distance);

                // println!(
                //     "radius: {} , mass: {}, force: {}",
                //     planet1.radius, mass1, force
                // );

                calculated_velocity +=
                    ((direction_to_entity2 * force) / mass1) * time.delta_seconds() * TIME_SPEED;
            }
        }

        changes.push(calculated_velocity);
    }

    for ((_e, _, mut planet), change) in planets_query.iter_mut().zip(changes) {
        // println!(
        //     "entity: '{}' velocity change: {}",
        //     _e.index(),
        //     change / GRAVITY_CONSTANT
        // );
        planet.velocity += change;
    }
}

fn move_planets(time: Res<Time>, mut planets_query: Query<(&mut Transform, &Planet)>) {
    for (mut transform, planet) in &mut planets_query {
        transform.translation += planet.velocity.extend(0.) * time.delta_seconds() * TIME_SPEED;
    }
}

#[derive(Component)]
struct Planet {
    radius: f32,
    density: f32,
    velocity: Vec2,
}

#[derive(Bundle)]
struct PlanetBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    planet: Planet,
}

impl PlanetBundle {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        // p_shape: impl Into<Mesh>,
        radius: f32,
        density: f32,
        color: Color,
        transform: Transform,
        initial_velocity: Option<Vec2>,
    ) -> Self {
        PlanetBundle {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: radius * SCALE,
                })),
                material: materials.add(color),
                transform,
                ..default()
            },
            planet: Planet {
                radius,
                density,
                velocity: initial_velocity.unwrap_or(Vec2::ZERO),
            },
        }
    }
}
