use bevy::prelude::*;


#[derive(Component)]
pub struct BackGroundWall {
    top_border: f32,
    bottom_border: f32,
    left_border: f32,
    right_border: f32,
}

impl BackGroundWall  {
    pub fn exclude_check(&self, &position: &Vec3) -> bool {
        if position.x < self.left_border ||
            position.x > self.right_border ||
            position.y < self.bottom_border ||
            position.y > self.top_border
        {
            return true;
        } else {
            return false;
        }
    }
}

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
    .insert(BackGroundWall {
        top_border: 1080.0,
        bottom_border: 0.0,
        left_border: 0.0,
        right_border: 1980.0
    });
}