use bevy::{prelude::*, input};

use crate::physical;

#[derive(Component)]
pub struct Board;

pub fn spawn_board(mut commands: Commands, assets_server: Res<AssetServer>) { 
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0,200.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -200.0, 1.0),
            ..default()
        },
        texture: assets_server.load("sprites/Sprite-0001.png"),
        ..default()
    })
    .insert(Board)
    .insert(physical::Velocity {
        x: 0.0,
        y: 0.0,
    })
    .insert(physical::AABBCollideBox {
        height: 70.0,
        width: 160.0,
        platform: false,
        ..default()
    });
}

pub fn board_movement (mut query: Query<&mut physical::Velocity, With<Board>>, keyboard_input: Res<Input<input::keyboard::KeyCode>>) {
    
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity.y = 5.;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity.y = - 5.;
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity.x = - 5.;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity.x = 5.;
        }
    }
}