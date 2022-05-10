use bevy::prelude::*;


#[derive(Component)]
pub struct Ball{
    pub speed: Vec2,
}

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
    .insert(Ball {
        speed: Vec2::new(0.0, -3.0)
    });
}

pub fn ball_movement (mut query: Query<(&mut GlobalTransform, &Ball), With<Ball>>) {
    for (mut global_transform, ball) in query.iter_mut() {
        global_transform.translation.x += ball.speed.x;
        global_transform.translation.y += ball.speed.y;
    }
}