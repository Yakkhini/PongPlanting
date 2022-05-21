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

use crate::{appstate, physical};

#[derive(Component)]
pub struct BackGroundWall;

#[derive(Component)]
pub struct TopWall;

#[derive(Component)]
pub struct BottomWall;

#[derive(Component)]
pub struct LeftWall;

#[derive(Component)]
pub struct RightWall;

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
        .insert(TopWall)
        .insert(Name::new("TopWall"))
        .insert(physical::AABBCollideBox {
            height: 20.0,
            width: 1980.0,
            platform: true,
            ..default()
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
        .insert(BottomWall)
        .insert(Name::new("BottomWall"))
        .insert(physical::AABBCollideBox {
            height: 20.0,
            width: 1980.0,
            platform: true,
            ..default()
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
        .insert(LeftWall)
        .insert(Name::new("LeftWall"))
        .insert(physical::AABBCollideBox {
            height: 1080.0,
            width: 20.0,
            platform: true,
            ..default()
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
        .insert(RightWall)
        .insert(Name::new("RightWall"))
        .insert(physical::AABBCollideBox {
            height: 1080.0,
            width: 20.0,
            platform: true,
            ..default()
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
