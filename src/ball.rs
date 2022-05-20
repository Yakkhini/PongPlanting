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
pub struct Ball;

pub fn spawn_ball(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..default()
            },
            texture: assets_server.load("sprites/Ball-texture.png"),
            ..default()
        })
        .insert(Ball)
        .insert(physical::Velocity { x: 20.0, y: -20.0 })
        .insert(physical::Touch { check: false })
        .insert(physical::AABBCollideBox {
            height: 30.0,
            width: 30.0,
            platform: false,
            ..default()
        });
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(appstate::AppState::InGame).with_system(spawn_ball));
    }
}
