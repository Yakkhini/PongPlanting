use bevy::prelude::*;

use crate::{physical, appstate};


#[derive(Component)]
pub struct Ball;

pub fn spawn_ball (mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
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
    .insert(physical::Velocity {
        x: 5.0,
        y: -5.0,
    })
    .insert(physical::AABBCollideBox {
        height: 30.0,
        width:30.0,
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