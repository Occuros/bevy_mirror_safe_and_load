//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_mod_component_mirror::rapier_mirrors::ColliderMirror;
use bevy_mod_component_mirror::RapierMirrorsPlugins;
use bevy_rapier3d::prelude::*;
use bevy::core::Name;

#[derive(Component)]
struct Cube {}

#[derive(Component)]
struct AllReady{}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(RapierMirrorsPlugins)
        .register_type::<ColliderMirror>()
        .add_startup_system(setup)
        .add_system(mark_as_dont_safe.after(setup).run_if(should_run))

        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
        .insert(Collider::cuboid(0.50, 0.50, 0.50))
        .insert(Cube{})
        .insert(Name::new("Cube"));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(AllReady{});
}

fn should_run(all_ready: Query<Entity, With<AllReady>>) -> bool {
    all_ready.get_single().is_ok()
}
fn mark_as_dont_safe(mut commands: Commands,
                     all_ready: Query<Entity, With<AllReady>>,
                     query: Query<Entity, (Without<Cube>, Without<Cube>)>,
) {
    let all_ready_entity = all_ready.single();


    for e in query.iter() {
        println!("we add not in scene for {}", e.index());
        commands.entity(e).insert(NotInScene);
    }

    commands.entity(all_ready_entity).despawn_recursive();

}