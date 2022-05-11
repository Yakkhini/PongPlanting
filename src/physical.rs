use bevy::{prelude::*, sprite::collide_aabb};

use crate::{ball};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Border {
    pub top_border: f32,
    pub bottom_border: f32,
    pub left_border: f32,
    pub right_border: f32,
}

#[derive(Component)]
pub struct AABBCollideBox {
    pub x_min: f32, 
    pub x_max: f32, 
    pub y_min: f32, 
    pub y_max: f32,
}

impl Default for AABBCollideBox {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
        }
    }

}

pub struct PhysicalPlugin;
impl Plugin for PhysicalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement);
        app.add_system(ball_collide);
        app.add_system(collide_box_update);
    }
}

fn movement(mut query: Query<(&mut GlobalTransform, &Velocity)>) {
    for (mut global_transform, volocity) in query.iter_mut() {
        global_transform.translation.x += volocity.x;
        global_transform.translation.y += volocity.y;
    }

}

fn collide_box_update (
    mut query: Query<(&mut AABBCollideBox, &GlobalTransform, &Sprite)>
) {
    for (mut collide_box, global_transform, sprite) in query.iter_mut() {
        let (translation, size) = (global_transform.translation, sprite.custom_size.unwrap()); 
        collide_box.x_min = translation.x - size.x * 0.5;
        collide_box.x_max = translation.x + size.x * 0.5;
        collide_box.y_min = translation.y - size.y * 0.5;
        collide_box.y_max = translation.y + size.y * 0.5;
    }
}

fn ball_collide(
    mut query_ball: Query<(&AABBCollideBox, &mut GlobalTransform, &Sprite), With<ball::Ball>>,
    query_others: Query<(&AABBCollideBox, &GlobalTransform, &Sprite), Without<ball::Ball>>
) {
    for (other_collide_box, other_global_transform, other_sprite) in query_others.iter() {
        match collide_aabb::collide(
            query_ball.single().1.translation,
            query_ball.single().2.custom_size.unwrap(),
            other_global_transform.translation,
            other_sprite.custom_size.unwrap()
        ).unwrap_or_else(|| collide_aabb::Collision::Inside) {
            collide_aabb::Collision::Left => {
                query_ball.single_mut().1.translation.x += other_collide_box.x_min - query_ball.single().0.x_max;
            },
            collide_aabb::Collision::Right => {
                query_ball.single_mut().1.translation.x += other_collide_box.x_max - query_ball.single().0.x_min;
            },
            collide_aabb::Collision::Top => {
                query_ball.single_mut().1.translation.y += other_collide_box.y_max - query_ball.single().0.y_min;
            },
            collide_aabb::Collision::Bottom => {
                query_ball.single_mut().1.translation.y += other_collide_box.y_min - query_ball.single().0.y_max;
            },
            collide_aabb::Collision::Inside => {
                println!("God What happend??");
            },
        }
    }
}
