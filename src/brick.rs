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

use crate::{appstate, map};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Brick;

fn spawn_bricks(
    assets_server: Res<AssetServer>,
    mut commands: Commands,
    bricks: Query<(&Brick, &map::GridLocation, Entity), Without<Sprite>>,
) {
    for (_brick, grid_location, e) in bricks.iter() {
        let name = "Brick".to_string() + &e.id().to_string();
        commands
            .entity(e)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 72.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        grid_location.x as f32 * 128.0,
                        grid_location.y as f32 * 72.0,
                        2.0,
                    ),
                    ..default()
                },
                texture: assets_server.load("sprites/brick.png"),
                ..default()
            })
            .insert(Name::new(name))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(64.0, 36.0, 0.0),
                border_radius: Some(0.0),
            })
            .insert(Velocity::from_linear(Vec3::X * 2.0))
            .insert(PhysicMaterial {
                restitution: 0.8,
                friction: 10.0,
                density: 10.0,
                ..Default::default()
            })
            .insert(RotationConstraints::lock());
    }
}

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Brick>();
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame).with_system(spawn_bricks),
        );
    }
}
