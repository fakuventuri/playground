// mod trail_plugin;
mod common;
mod utils;

use std::f32::consts::PI;

use bevy::prelude::*;
use common::CommonPlugin;
// use trail_plugin::{Trailed, Trailplugin};
use utils::*;

const SCALE: f32 = 7. / 1000.;
const DISTANCE_SHORTENER: f32 = 10.; // for better display
const TIME_SPEED: f32 = 233280.; // moon orbit 27 days = 2332800s / 10 for 10 sec rotation

fn main() {
    App::new() //
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        // .add_plugins(Trailplugin)
        .add_systems(Startup, (setup, setup_planets))
        .add_systems(FixedUpdate, (calculate_velocity, move_planets).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 750.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0,
    });

    // Point Light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(0.0, 250.0, 0.0),
            point_light: PointLight {
                intensity: 1_000_000_000.0,
                color: Color::WHITE,
                shadows_enabled: true,
                range: 10000.,
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

    commands.spawn(PlanetBundle::new(
        &mut meshes,
        6378.,
        5.51,
        materials.add(Color::BLUE),
        Transform::from_xyz(0.0, 0.0, 0.0),
        None,
    ));

    commands
        .spawn(PlanetBundle::new(
            &mut meshes,
            1737.4,
            3.34,
            materials.add(StandardMaterial{
                base_color: Color::WHITE,
                ..Default::default()
            }),
            Transform::from_xyz(0., 384400. * SCALE / DISTANCE_SHORTENER, 0.0),
            Some(Vec3::new(1., 0., 0.) * 1022. / 233280. / 10. * 2.),
        ))
        // .insert(Trailed::new())
        ;

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     3000.,
    //     10.,
    //     materials.add(Color::SILVER),
    //     Transform::from_xyz(-275.0, 13.0, 10.0),
    //     None,
    // ));

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     3000.,
    //     10.,
    //     materials.add(Color::SILVER),
    //     Transform::from_xyz(175.0, -13.0, 150.0),
    //     None,
    // ));

    // commands.spawn(PlanetBundle::new(
    //     &mut meshes,
    //     3000.,
    //     10.,
    //     materials.add(Color::SILVER),
    //     Transform::from_xyz(-175.0, -130.0, -100.0),
    //     None,
    // ));
}

fn calculate_velocity(
    time: Res<Time>,
    mut planets_query: Query<(Entity, &Transform, &mut Planet)>,
) {
    let mut changes: Vec<Vec3> = Vec::new();

    for (entity1, transform1, planet1) in &planets_query {
        let mut calculated_velocity = Vec3::ZERO;

        for (entity2, transform2, planet2) in &planets_query {
            if !entity1.eq(&entity2) {
                let direction_to_entity2 =
                    (transform2.translation - transform1.translation).normalize_or_zero();

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

    for ((_e, _t, mut planet), change) in planets_query.iter_mut().zip(changes) {
        // println!(
        //     "entity: '{}' velocity change: {}",
        //     _e.index(),
        //     change / GRAVITY_CONSTANT
        // );
        // println!("entity: '{}' pos: {}", _e.index(), _t.translation);
        planet.velocity += change;
    }
}

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
                mesh: meshes.add(Sphere::new(radius * SCALE).mesh().ico(5).unwrap()),
                material,
                transform: transform.with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            planet: Planet {
                radius,
                density,
                velocity: initial_velocity.unwrap_or(Vec3::ZERO),
            },
        }
    }
}
