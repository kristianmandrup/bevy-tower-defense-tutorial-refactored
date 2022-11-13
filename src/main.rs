use bevy::{pbr::NotShadowCaster, prelude::*, utils::FloatOrd};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_mod_picking::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

pub struct GameAssets {
    bullet_scene: Handle<Scene>,
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    potato_tower_scene: Handle<Scene>,
    potato_scene: Handle<Scene>,
    cabbage_tower_scene: Handle<Scene>,
    cabbage_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
}

mod bullet;
mod target;
mod tower;
mod camera;

pub use tower::*;
pub use bullet::*;
pub use target::*;
pub use camera::*;
use tower::{tower::TowerPlugin, tower_type::TowerType};

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Bevy Tower Defense".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Inspector Setup
        .add_plugin(WorldInspectorPlugin::new())
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        // Our Systems
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .register_inspectable::<TowerType>()
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_system(camera_controls)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Tomato.glb#Scene0"),
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        potato_tower_scene: assets.load("PotatoTower.glb#Scene0"),
        potato_scene: assets.load("Potato.glb#Scene0"),
        cabbage_tower_scene: assets.load("CabbageTower.glb#Scene0"),
        cabbage_scene: assets.load("Cabbage.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

/* Selection testing system
fn what_is_selected(selection: Query<(&Name, &Selection)>) {
    for (name, selection) in &selection {
        if selection.selected() {
            info!("{}", name);
        }
    }
}
*/

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.3, 0.6, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    for i in 0..10 {
        for j in 0..2 {
            commands
                .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
                    4.0 * i as f32 + j as f32,
                    0.8,
                    8.0 * j as f32,
                )))
                .insert(Name::new("Tower_Base"))
                .insert(meshes.add(shape::Capsule::default().into()))
                .insert(Highlighting {
                    initial: default_collider_color.clone(),
                    hovered: Some(selected_collider_color.clone()),
                    pressed: Some(selected_collider_color.clone()),
                    selected: Some(selected_collider_color.clone()),
                })
                .insert(default_collider_color.clone())
                .insert(NotShadowCaster)
                .insert_bundle(PickableBundle::default())
                .with_children(|commands| {
                    commands.spawn_bundle(SceneBundle {
                        scene: game_assets.tower_base_scene.clone(),
                        transform: Transform::from_xyz(0.0, -0.8, 0.0),
                        ..Default::default()
                    });
                });
        }
    }

    for i in 1..25 {
        commands
            .spawn_bundle(SceneBundle {
                scene: game_assets.target_scene.clone(),
                transform: Transform::from_xyz(-4.0 * i as f32, 0.4, 2.5),
                ..Default::default()
            })
            .insert(Target { speed: 0.6 })
            .insert(Health { value: 5 })
            .insert(Name::new("Target"));
    }

    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert_bundle(PickingCameraBundle::default());
}
