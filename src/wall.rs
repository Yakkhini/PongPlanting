use bevy::prelude::*;

use crate::physical;


#[derive(Component)]
pub struct BackGroundWall;

pub fn spawn_bgwall(
    mut commands: Commands,
    assets_sever: Res<AssetServer>
) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(1980.0, 1080.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        texture: assets_sever.load("sprites/bg.png"),
        ..default()
    })
    .insert(BackGroundWall)
    .insert(physical::Border {
        top_border: 0.0,
        bottom_border: 0.0,
        left_border: 0.0,
        right_border: 0.0,
    });
}