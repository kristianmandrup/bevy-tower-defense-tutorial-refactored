use bevy::prelude::*;

use crate::*;
use super::tower::*;


#[derive(Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType {
    Tomato,
    Potato,
    Cabbage,
}

impl TowerType {
    fn timer(&self, duration: f32) -> Timer {
        Timer::from_seconds(duration, true)
    }

    fn create_tower(&self, duration: f32) -> Tower {     
        Tower {
            shooting_timer: self.timer(duration),
            bullet_offset: self.offset(),
        }
    }

    fn create_bullet(&self, direction: Vec3, speed: f32) -> Bullet {     
        Bullet {
            direction,
            speed,
        }
    }

    fn offset(&self) -> Vec3 {
        Vec3::new(0.0, 0.6, 0.0)
    }

    pub fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        
        match self {
            TowerType::Tomato => (
                assets.tomato_tower_scene.clone(),
                self.create_tower(1.0)
            ),
            TowerType::Potato => (
                assets.potato_tower_scene.clone(),
                self.create_tower( 2.0)
            ),
            TowerType::Cabbage => (
                assets.cabbage_tower_scene.clone(),
                self.create_tower( 3.0)
            ),
        }
    }

    pub fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::Tomato => (
                assets.tomato_scene.clone(),
                self.create_bullet(direction, 3.5)
            ),
            TowerType::Potato => (
                assets.potato_scene.clone(),
                self.create_bullet(direction, 6.5)
            ),
            TowerType::Cabbage => (
                assets.cabbage_scene.clone(),
                self.create_bullet(direction, 2.5)
            ),
        }
    }
}