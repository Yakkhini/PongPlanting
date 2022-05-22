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

use bevy::{core::FixedTimestep, input, prelude::*};
use heron::prelude::*;

use crate::appstate;

#[derive(Component)]
pub struct Board;

pub fn spawn_board(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -400.0, 2.0),
                ..default()
            },
            texture: assets_server.load("sprites/Sprite-0001.png"),
            ..default()
        })
        .insert(Board)
        .insert(Name::new("Board"))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(50.0, 15.0, 0.0),
            border_radius: Some(0.0),
        })
        .insert(Velocity::from_linear(Vec3::X * 2.0))
        .insert(PhysicMaterial { restitution: 0.8, friction:10.0, density:10.0, ..Default::default() })
        .insert(RotationConstraints::lock());
}

pub fn board_movement(
    mut query: Query<&mut Velocity, With<Board>>,
    keyboard_input: Res<Input<input::keyboard::KeyCode>>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity.linear.y = 160.;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity.linear.y = -160.;
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity.linear.x = -700.;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity.linear.x = 700.;
        }
        velocity.linear.x = velocity.linear.x * 0.6;
        velocity.linear.y = velocity.linear.y * 0.6;
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(appstate::AppState::InGame)
                .with_system(spawn_board.after("spawn wall")),
        );
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame)
                .with_run_criteria(FixedTimestep::step(0.08))
                .with_system(board_movement),
        );
    }
}
