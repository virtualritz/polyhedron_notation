use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::{
        light_consts, DirectionalLight, DirectionalLightBundle,
        DirectionalLightShadowMap, PbrBundle, StandardMaterial,
    },
    prelude::{
        Component, PluginGroup, Query, Res, Time, Update, Window, WindowPlugin,
    },
    render::{mesh::Mesh, view::Msaa},
    transform::components::Transform,
    utils::default,
    window::WindowMode,
    DefaultPlugins,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use polyhedron_ops::Polyhedron;
use std::f32::consts::TAU;

#[cfg(feature = "console")]
mod console;
#[cfg(feature = "console")]
use console::prelude::*;

#[derive(Component)]
pub struct RootPolyhedron {
    speed: f32,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Sample4)
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: true,
                mode: WindowMode::BorderlessFullscreen,
                visible: true,
                title: "Polyhedron-Ops".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_polyhedron);

    #[cfg(feature = "console")]
    app.add_plugins(ConsolePlugin)
        .add_console_command::<RenderCommand, _>(render_command);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // gapcD
    let polyhedron = Polyhedron::dodecahedron()
        .c()
        .p()
        .a()
        .g()
        .normalize()
        .finalize();

    assert_eq!(polyhedron.name(), "gapcD");

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(polyhedron)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                double_sided: true,
                metallic: 0.9,
                diffuse_transmission: 0.1,
                ..Default::default()
            }),
            ..Default::default()
        },
        RootPolyhedron { speed: 0.1 },
    ));

    // Directional Light.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(1.0, 4.0, 2.0)),
        ..Default::default()
    });

    // Camera.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 1.0, 2.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn rotate_polyhedron(
    mut polyhedra: Query<(&mut Transform, &RootPolyhedron)>,
    timer: Res<Time>,
) {
    for (mut transform, polyhedron) in &mut polyhedra {
        transform.rotate_y(polyhedron.speed * TAU * timer.delta_seconds());
    }
}
