use bevy::{prelude::*, input, sprite::Anchor};

#[derive(Component)]
pub struct Board;

pub fn spawn_board(mut commands: Commands, assets_server: Res<AssetServer>) { 
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0,200.0)),
            anchor: Anchor::TopCenter,
            ..default()
        },
        texture: assets_server.load("sprites/Sprite-0001.png"),
        ..default()
    })
    .insert(Board);
}

pub fn board_movement (mut query: Query<&mut GlobalTransform, With<Board>>, keyboard_input: Res<Input<input::keyboard::KeyCode>>) {
    
    for mut global_transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) | keyboard_input.pressed(KeyCode::W) {
            global_transform.translation.y += 5.;
        }
        if keyboard_input.pressed(KeyCode::Down) | keyboard_input.pressed(KeyCode::S) {
            global_transform.translation.y -= 5.;
        }
        if keyboard_input.pressed(KeyCode::Left) | keyboard_input.pressed(KeyCode::A) {
            global_transform.translation.x -= 5.;
        }
        if keyboard_input.pressed(KeyCode::Right) | keyboard_input.pressed(KeyCode::D) {
            global_transform.translation.x += 5.;
        }
    }
}