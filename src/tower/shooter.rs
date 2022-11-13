use bevy::prelude::*;

use crate::*;
use super::tower::*;
use super::tower_type::*;

// #[derive(Component, Default)]
pub struct TowerShooter<'a> {
    entity: Entity, 
    tower_type: &'a TowerType, 
    transform: &'a GlobalTransform
}
impl<'a> TowerShooter<'a> {
    pub fn new(entity: Entity, tower_type: &'a TowerType, transform: &'a GlobalTransform) -> Self {
        TowerShooter {
            entity,
            tower_type,
            transform
        }
    }

    fn get_bullet_spawn(&self, tower: &Tower) -> Vec3 {
        self.transform.translation() + tower.bullet_offset
    }        

    fn distance_from(&self, target: Vec3, bullet_spawn: Vec3) -> FloatOrd {
        FloatOrd(Vec3::distance(target, bullet_spawn))
    }
    
    fn get_direction(&self, tower: &Tower, targets: &Query<&GlobalTransform, With<Target>>) -> Option<Vec3> {
        let bullet_spawn: Vec3 = self.get_bullet_spawn(tower);
        targets
            .iter()
            .min_by_key(|target_transform| {
                // FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                self.distance_from(target_transform.translation(), bullet_spawn)
            })
            .map(|closest_target| closest_target.translation() - bullet_spawn)
    }
    
    pub fn shoot_from(&self, commands: &mut Commands, tower: &Tower, targets: &Query<&GlobalTransform, With<Target>>, bullet_assets: &GameAssets) {
        let ctx = (commands, tower);
        if let Some(direction) = self.get_direction(tower, targets) {    
            self.shoot_direction(ctx, direction, bullet_assets)
        }
        else { return };
    }

    fn shoot_direction(&self, ctx: (&mut Commands,  &Tower), direction: Vec3, bullet_assets: &GameAssets) {
        let (model, bullet) = self.tower_type.get_bullet(direction, &bullet_assets);
        self.spawn(ctx, model, bullet)
    }

    fn bullet_lifetime(&self) -> Lifetime {
        Lifetime {
            timer: Timer::from_seconds(10.0, false),
        }        
    }

    fn scene_bundle(&self, tower: &Tower, scene: Handle<Scene>) -> SceneBundle {
        SceneBundle {
            scene,
            transform: Transform::from_translation(tower.bullet_offset),
            ..Default::default()
        }        
    }

    fn spawn(&self, ctx: (&mut Commands,  &Tower), scene: Handle<Scene>, bullet: Bullet) {
        let (commands, tower) = ctx;
        commands.entity(self.entity).with_children(|commands| {
            commands
                .spawn_bundle(self.scene_bundle(&tower, scene))
                .insert(self.bullet_lifetime())
                .insert(bullet)
                .insert(Name::new("Bullet"));
        });        
    }
}