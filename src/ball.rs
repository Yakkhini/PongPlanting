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
        .insert(Name::new("Ball"))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 25.0 })
        .insert(PhysicMaterial {
            restitution: 1.0,
            friction: 0.0,
            density: 1.0,
            ..Default::default()
        })
        .insert(Velocity::from_linear(Vec3::X * 2.0));
}

fn ball_movement(mut query: Query<&mut Velocity, With<Ball>>) {
    let mut velocity = query.single_mut();
    if velocity.linear.x.powi(2) + velocity.linear.y.powi(2) < 370.0 * 370.0 {
        let k1 = velocity.linear.x / (velocity.linear.x.powi(2) + velocity.linear.y.powi(2)).sqrt();
        let k2 = velocity.linear.y / (velocity.linear.x.powi(2) + velocity.linear.y.powi(2)).sqrt();
        velocity.linear.x = k1 * 400.0;
        velocity.linear.y = k2 * 400.0;
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(appstate::AppState::InGame)
                .with_system(spawn_ball.after("spawn wall")),
        );
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame).with_system(ball_movement),
        );
    }
}
