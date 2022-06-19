/*
Copyright (c) 2022 Yakkhini
Planting Pong is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
         http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
*/

use bevy::prelude::*;
use heron::prelude::*;

use crate::appstate;

#[derive(Component)]
pub struct Wall;

pub fn spawn_top_wall(
    mut commands: Commands,
    //assets_sever: Res<AssetServer>
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1980.0, 20.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 550.0, 0.0),
                ..default()
            },
            global_transform: GlobalTransform {
                translation: Vec3::new(0.0, 550.0, 0.0),
                ..default()
            },
            //texture: assets_sever.load("sprites/bg.png"),
            ..default()
        })
        .insert(Wall)
        .insert(Name::new("TopWall"))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(990.0, 10.0, 0.0),
            border_radius: Some(0.0),
        });
}

pub fn spawn_bottom_wall(
    mut commands: Commands,
    //assets_sever: Res<AssetServer>
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1980.0, 20.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -550.0, 0.0),
                ..default()
            },
            global_transform: GlobalTransform {
                translation: Vec3::new(0.0, -550.0, 0.0),
                ..default()
            },
            //texture: assets_sever.load("sprites/bg.png"),
            ..default()
        })
        .insert(Wall)
        .insert(Name::new("BottomWall"))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(990.0, 10.0, 0.0),
            border_radius: Some(0.0),
        });
}

pub fn spawn_left_wall(
    mut commands: Commands,
    //assets_sever: Res<AssetServer>
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 1080.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-1000.0, 0.0, 0.0),
                ..default()
            },
            global_transform: GlobalTransform {
                translation: Vec3::new(-1000.0, 0.0, 0.0),
                ..default()
            },
            //texture: assets_sever.load("sprites/bg.png"),
            ..default()
        })
        .insert(Wall)
        .insert(Name::new("LeftWall"))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(10.0, 540.0, 0.0),
            border_radius: Some(0.0),
        });
}

pub fn spawn_right_wall(
    mut commands: Commands,
    //assets_sever: Res<AssetServer>
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 1080.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1000.0, 0.0, 0.0),
                ..default()
            },
            global_transform: GlobalTransform {
                translation: Vec3::new(1000.0, 0.0, 0.0),
                ..default()
            },
            //texture: assets_sever.load("sprites/bg.png"),
            ..default()
        })
        .insert(Wall)
        .insert(Name::new("RightWall"))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(10.0, 540.0, 0.0),
            border_radius: Some(0.0),
        });
}

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(appstate::AppState::InGame)
                .label("spawn wall")
                .with_system(spawn_top_wall)
                .with_system(spawn_bottom_wall)
                .with_system(spawn_right_wall)
                .with_system(spawn_left_wall),
        );
    }
}
