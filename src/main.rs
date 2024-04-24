// mod trail_plugin;
mod common;
mod hud;
mod player;
mod utils;

use std::f32::consts::PI;

use bevy::prelude::*;
use common::CommonPlugin;
use hud::HUDPlugin;
use player::PlayerPlugin;
use rand::Rng;
// use trail_plugin::{Trailed, Trailplugin};
use utils::*;

const SIZE_SCALE: f32 = 7. / 1000.;
const DISTANCE_SCALE: f32 = 10.; // for better display
const TIME_SPEED: f32 = 2332.800; // moon orbit 27 days = 2332800s / 10 for 10 sec rotation // new alg 2332.800

fn main() {
    App::new() //
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(HUDPlugin)
        // .add_plugins(Trailplugin)
        .add_systems(Startup, (setup, setup_planets))
        .add_systems(
            FixedUpdate,
            (velocity_verlet, calculate_acceleration_velocity).chain(),
        )
        // .add_systems(FixedUpdate, (calculate_velocity, move_planets).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0,
    });

    // Point Light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(0.0, 250.0, 0.0),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            point_light: PointLight {
                intensity: 1_000_000_000.0,
                color: Color::WHITE,
                shadows_enabled: true,
                range: 15000.,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(5.).mesh().ico(5).unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    // emissive: Color::rgb(5., 5., 5.),
                    unlit: true,
                    ..default()
                }),
                ..default()
            });
        });
}

fn setup_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut images: ResMut<Assets<Image>>,
) {
    // let debug_material = StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // };

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     6378.,
    //     5.51,
    //     materials.add(Color::BLUE),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     None,
    // ));

    // commands
    //     .spawn(PlanetBundle::new(
    //         &mut meshes,
    //         1737.4,
    //         3.34,
    //         materials.add(StandardMaterial{
    //             base_color: Color::WHITE,
    //             ..Default::default()
    //         }),
    //         Transform::from_xyz(0., 384400. * SIZE_SCALE / DISTANCE_SCALE, 0.0),
    //         Some(Vec3::new(1., 0., 0.) * 1022. * SIZE_SCALE / DISTANCE_SCALE / 9.), // Some(Vec3::new(1., 0., 0.) * 1022. / 233280. / 10. * 2.)
    //     ))
    //     // .insert(Trailed::new())
    //     ;

    for _ in 0..500 {
        let x = rand::thread_rng().gen_range(-400f32..400f32);
        let y = rand::thread_rng().gen_range(-400f32..400f32);
        let z = rand::thread_rng().gen_range(-400f32..400f32);

        commands.spawn(PlanetBundle::new(
            &mut meshes,
            300.,
            100.,
            materials.add(Color::SILVER),
            Transform::from_xyz(x, y, z),
            None,
        ));
    }

    commands.spawn(PlanetBundle::new(
        &mut meshes,
        3000.,
        10.,
        materials.add(Color::RED),
        Transform::from_xyz(-275.0, 13.0, 10.0),
        None,
    ));

    commands.spawn(PlanetBundle::new(
        &mut meshes,
        3000.,
        10.,
        materials.add(Color::GREEN),
        Transform::from_xyz(175.0, -13.0, 150.0),
        None,
    ));

    commands.spawn(PlanetBundle::new(
        &mut meshes,
        3000.,
        10.,
        materials.add(Color::BLUE),
        Transform::from_xyz(-175.0, -130.0, -100.0),
        None,
    ));

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            // metallic: 1.,
            cull_mode: None,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0., -450., 0.),
        ..default()
    });
}

fn calculate_velocity(time: Res<Time>, mut planets_query: Query<(&Transform, &mut Planet)>) {
    let mut iter = planets_query.iter_combinations_mut();

    while let Some([(transform1, mut planet1), (transform2, mut planet2)]) = iter.fetch_next() {
        let direction_to_entity2 =
            (transform2.translation - transform1.translation).normalize_or_zero();

        let distance = transform1.translation.distance(transform2.translation);

        let mass1 = ((4. / 3.) * PI * planet1.radius.powi(3)) * planet1.density;
        let mass2 = ((4. / 3.) * PI * planet2.radius.powi(3)) * planet2.density;

        let mut force = calculate_force(mass1, mass2, distance / SIZE_SCALE * DISTANCE_SCALE);

        if distance < (planet1.radius + planet2.radius) * SIZE_SCALE {
            force = force * (distance / (planet1.radius + planet2.radius) * SIZE_SCALE);
        }

        planet1.velocity +=
            ((direction_to_entity2 * force) / mass1) * time.delta_seconds() * TIME_SPEED;

        planet2.velocity +=
            ((-direction_to_entity2 * force) / mass2) * time.delta_seconds() * TIME_SPEED;
    }
}

fn calculate_acceleration_velocity(
    time: Res<Time>,
    mut planets_query: Query<(&Transform, &mut Planet)>,
) {
    let dt = time.delta_seconds() * TIME_SPEED;
    let mut iter = planets_query.iter_combinations_mut();

    while let Some([(transform1, mut planet1), (transform2, mut planet2)]) = iter.fetch_next() {
        let delta = transform2.translation - transform1.translation;
        let distance_sq: f32 = delta.length_squared();

        let mass1 = ((4. / 3.) * PI * planet1.radius.powi(3)) * planet1.density;
        let mass2 = ((4. / 3.) * PI * planet2.radius.powi(3)) * planet2.density;

        let mut f = GRAVITY_CONSTANT / (distance_sq / (SIZE_SCALE * DISTANCE_SCALE).powi(2));

        let distance = distance_sq.sqrt();
        if distance < (planet1.radius + planet2.radius) * SIZE_SCALE {
            f = f * (distance / (planet1.radius + planet2.radius) * SIZE_SCALE);
        }

        // Collision
        if distance_sq < ((planet1.radius + planet2.radius) * SIZE_SCALE).powf(2.) {
            // f = f * (distance_sq / ((planet1.radius + planet2.radius) * SIZE_SCALE).powi(2));
            f = -f;
        }

        let force_unit_mass = delta.normalize_or_zero() * f;

        let acc1 = force_unit_mass * mass2;
        let acc2 = -(force_unit_mass * mass1);

        planet1.velocity = planet1.velocity + (planet1.acceleration + acc1) * (dt * 0.5);

        planet2.velocity = planet2.velocity + (planet2.acceleration + acc2) * (dt * 0.5);

        planet1.acceleration = acc1;
        planet2.acceleration = acc2;
    }
}

// fn integrate(time: Res<Time>, mut planets_query: Query<(&mut Transform, &mut Planet)>) {
//     let dt_sq = time.delta_seconds() * time.delta_seconds() * TIME_SPEED * TIME_SPEED;
//     for (mut transform, mut planet) in &mut planets_query {
//         // verlet integration
//         // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)

//         let new_pos = transform.translation * 2.0 - planet.last_pos + planet.acceleration * dt_sq;
//         planet.acceleration = Vec3::ZERO;
//         planet.last_pos = transform.translation;
//         transform.translation = new_pos;
//     }
// }

fn velocity_verlet(time: Res<Time>, mut planets_query: Query<(&mut Transform, &Planet)>) {
    let dt = time.delta_seconds() * TIME_SPEED;

    for (mut transform, planet) in &mut planets_query {
        transform.translation += planet.velocity * dt + planet.acceleration * (dt * dt * 0.5);
    }
}

#[allow(dead_code)]
fn move_planets(time: Res<Time>, mut planets_query: Query<(&mut Transform, &Planet)>) {
    for (mut transform, planet) in &mut planets_query {
        transform.translation += planet.velocity * time.delta_seconds() * TIME_SPEED;
    }
}

#[derive(Component)]
struct Planet {
    radius: f32,
    density: f32,
    velocity: Vec3,
    acceleration: Vec3,
    // last_pos: Vec3,
}

#[derive(Bundle)]
struct PlanetBundle {
    // material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pbr_bundle: PbrBundle,
    planet: Planet,
}

impl PlanetBundle {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        // materials: &mut ResMut<Assets<StandardMaterial>>,
        radius: f32,
        density: f32,
        material: Handle<StandardMaterial>,
        transform: Transform,
        initial_velocity: Option<Vec3>,
    ) -> Self {
        PlanetBundle {
            // material_mesh_bundle: MaterialMesh2dBundle {
            //     mesh: Mesh2dHandle(meshes.add(Circle {
            //         radius: radius * SCALE,
            //     })),
            //     material: materials.add(color),
            //     transform,
            //     ..default()
            // },
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Sphere::new(radius * SIZE_SCALE).mesh().ico(5).unwrap()),
                material,
                transform: transform.with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            planet: Planet {
                radius,
                density,
                velocity: initial_velocity.unwrap_or(Vec3::ZERO),
                acceleration: Vec3::ZERO,
                // last_pos: transform.translation,
            },
        }
    }
}
