use bevy::prelude::*;

use crate::physical;


#[derive(Component)]
pub struct Ball;

pub fn spawn_ball (mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        texture: assets_server.load("sprites/Ball-texture.png"),
        ..default()
    })
    .insert(Ball)
    .insert(physical::Velocity {
        x: 5.0,
        y:-5.0,
    })
    .insert(physical::Border {
        top_border: 0.0,
        bottom_border: 0.0,
        left_border: 0.0,
        right_border: 0.0,
    });
}