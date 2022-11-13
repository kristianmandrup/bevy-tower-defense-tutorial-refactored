use bevy::prelude::*;

use crate::*;
use super::ui::*;
use super::shooter::*;
use super::tower_type::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}


pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build<'a>(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_system(tower_shooting)
            .add_system(tower_button_clicked)
            .add_system(create_ui_on_selection);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}


fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (entity, mut tower, tower_type, transform) in &mut towers {
        let tower_shooter = TowerShooter::new(entity, &tower_type, &transform);
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            tower_shooter.shoot_from(&mut commands, &tower, &targets, &bullet_assets)
        }
    }
}

pub fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType,
) -> Entity {
    let (tower_scene, tower) = tower_type.get_tower(assets);
    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new(format!("{:?}_Tower", tower_type)))
        .insert(tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn_bundle(SceneBundle {
                scene: tower_scene,
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..Default::default()
            });
        })
        .id()
}


