use bevy::prelude::*;

use crate::physical;


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

pub fn spawn_bgwall(
    mut commands: Commands,
    //assets_sever: Res<AssetServer>
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
        //texture: assets_sever.load("sprites/bg.png"),
        ..default()
    })
    .insert(BackGroundWall);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(1980.0, 10.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 545.0, 0.0),
            ..default()
        },
        //texture: assets_sever.load("sprites/bg.png"),
        ..default()
    })
    .insert(TopWall)
    .insert(physical::AABBCollideBox {
        height: 10.0,
        width: 1980.0,
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(1980.0, 10.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -545.0, 0.0),
            ..default()
        },
        //texture: assets_sever.load("sprites/bg.png"),
        ..default()
    })
    .insert(BottomWall)
    .insert(physical::AABBCollideBox {
        height: 10.0,
        width: 1980.0,
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(10.0, 1080.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-995.0, 0.0, 0.0),
            ..default()
        },
        //texture: assets_sever.load("sprites/bg.png"),
        ..default()
    })
    .insert(LeftWall)
    .insert(physical::AABBCollideBox {
        height: 1080.0,
        width: 10.0,
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(10.0, 1080.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(995.0, 0.0, 0.0),
            ..default()
        },
        //texture: assets_sever.load("sprites/bg.png"),
        ..default()
    })
    .insert(RightWall)
    .insert(physical::AABBCollideBox {
        height: 1080.0,
        width: 10.0,
        ..default()
    });
}