use bevy::prelude::*;

use crate::wall;

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

pub struct PhysicalPlugin;
impl Plugin for PhysicalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement);
        app.add_system(collide_wall);
        app.add_system(border_update);
    }
}

fn movement(mut query: Query<(&mut GlobalTransform, &Velocity)>) {
    for (mut global_transform, volocity) in query.iter_mut() {
        global_transform.translation.x += volocity.x;
        global_transform.translation.y += volocity.y;
    }

}

fn border_update(mut query: Query<(&mut Border, &GlobalTransform, &Sprite), Without<wall::BackGroundWall>>) {
    for mut item in query.iter_mut() {
        let (mut border, translation, size) = (item.0, item.1.translation, item.2.custom_size.unwrap());
        border.top_border = translation.y + 0.5 * size.y;
        border.bottom_border = translation.y - 0.5 * size.y;
        border.left_border = translation.x - 0.5 * size.x;
        border.right_border = translation.x + 0.5 * size.x;
        item.0 = border;
    }
}

fn collide_check(staff1: &Border, staff2: &Border) -> bool {
    if (staff1.left_border < staff2.right_border ||
        staff1.right_border > staff2.left_border) &&
        (staff1.bottom_border < staff2.top_border ||
        staff1.top_border > staff2.bottom_border) {
            return true;
    } else {
        return false;
    }
}

fn collide_wall (
    wall: Query<&Border, With<wall::BackGroundWall>>,
    mut board_or_ball: Query<(&mut Velocity, &mut Transform, &Border), Without<wall::BackGroundWall>>
) {
    for mut item in board_or_ball.iter_mut() {
        if !(collide_check(wall.single(), item.2)
        ) {
            item.1.translation.x -=  item.1.translation.x / (item.1.translation.x * item.1.translation.x);
            item.0.x = -item.0.x;
            item.0.y = -item.0.y;
        }
    }
}