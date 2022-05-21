/*
Copyright (c) 2022 Yakkhini
GLSL_Journey is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
         http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
*/

use bevy::{prelude::*, sprite::collide_aabb};

use crate::{appstate, ball, board};

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
pub struct Touch {
    pub check: bool,
}

#[derive(Component, Debug, PartialEq)]
pub struct AABBCollideBox {
    pub collsion_state: bool,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub height: f32,
    pub width: f32,
    pub platform: bool,
}

impl Default for AABBCollideBox {
    fn default() -> Self {
        Self {
            collsion_state: false,
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            height: 0.0,
            width: 0.0,
            platform: true,
        }
    }
}

struct CollisionEvent {
    subject: Entity,
    object: Entity,
    subject_transform: GlobalTransform,
    object_transform: GlobalTransform,
    collision_axis: String,
}

fn movement(mut query: Query<(&mut Transform, &Velocity, &AABBCollideBox)>, time: Res<Time>) {
    for (mut transform, volocity, collide_box) in query.iter_mut() {
        if collide_box.platform == false {
            transform.translation.x += volocity.x * time.delta_seconds() * 10.0;
            transform.translation.y += volocity.y * time.delta_seconds() * 10.0;
        }
    }
}

fn collide_box_update(mut query_box: Query<(&mut AABBCollideBox, &Transform)>) {
    for (mut collide_box, transform) in query_box.iter_mut() {
        let (translation, height, width) =
            (transform.translation, collide_box.height, collide_box.width);
        collide_box.x_min = translation.x - width * 0.5;
        collide_box.x_max = translation.x + width * 0.5;
        collide_box.y_min = translation.y - height * 0.5;
        collide_box.y_max = translation.y + height * 0.5;
        //println!("{:?}, {:?}", collide_box, translation);
    }
}

fn collide_check(
    &subject: &(&Mut<AABBCollideBox>, GlobalTransform, &Velocity, Entity),
    &object: &(&Mut<AABBCollideBox>, GlobalTransform, &Velocity, Entity),
    time: &Res<Time>,
) -> (Vec3, Vec3, String) {
    let mut result_subject = subject.1.translation.clone();
    let mut result_object = object.1.translation.clone();
    let mut axis = String::new();
    let check_result = collide_aabb::collide(
        subject.1.translation
            + Vec3::new(
                subject.2.x * time.delta_seconds() * 10.0,
                subject.2.y * time.delta_seconds() * 10.0,
                0.0,
            ),
        Vec2::new(subject.0.width, subject.0.height),
        object.1.translation
            + Vec3::new(
                object.2.x * time.delta_seconds() * 10.0,
                object.2.y * time.delta_seconds() * 10.0,
                0.0,
            ),
        Vec2::new(object.0.width, object.0.height),
    );

    match check_result {
        Some(_) => {
            let check_result = check_result.unwrap();
            match check_result {
                collide_aabb::Collision::Left => match (subject.0.platform, object.0.platform) {
                    (true, true) => {}
                    (true, false) => {
                        result_object.x += subject.0.x_max - object.0.x_min + 1.5;
                    }
                    (false, true) => {
                        result_subject.x += object.0.x_min - subject.0.x_max - 1.5;
                    }
                    (false, false) => {
                        result_object.x += 0.5 * (subject.0.x_max - object.0.x_min + 1.5);
                        result_subject.x += 0.5 * (object.0.x_min - subject.0.x_max - 1.5);
                    }
                },
                collide_aabb::Collision::Right => match (subject.0.platform, object.0.platform) {
                    (true, true) => {}
                    (true, false) => {
                        result_object.x += subject.0.x_min - object.0.x_max - 1.5;
                    }
                    (false, true) => {
                        result_subject.x += object.0.x_max - subject.0.x_min + 1.5;
                    }
                    (false, false) => {
                        result_object.x += 0.5 * (subject.0.x_min - object.0.x_max - 1.5);
                        result_subject.x += 0.5 * (object.0.x_max - subject.0.x_min + 1.5);
                    }
                },
                collide_aabb::Collision::Top => match (subject.0.platform, object.0.platform) {
                    (true, true) => {}
                    (true, false) => {
                        result_object.y += subject.0.y_min - subject.0.y_max - 1.5;
                    }
                    (false, true) => {
                        result_subject.y += object.0.y_max - subject.0.y_min + 1.5;
                    }
                    (false, false) => {
                        result_object.y += 0.5 * (subject.0.y_min - subject.0.y_max - 1.5);
                        result_subject.y += 0.5 * (object.0.y_max - subject.0.y_min + 1.5);
                    }
                },
                collide_aabb::Collision::Bottom => {
                    if subject.0.platform == false {
                        match (subject.0.platform, object.0.platform) {
                            (true, true) => {}
                            (true, false) => {
                                result_object.y += subject.0.y_max - subject.0.y_min + 1.5;
                            }
                            (false, true) => {
                                result_subject.y += object.0.y_min - subject.0.y_max - 1.5;
                            }
                            (false, false) => {
                                result_object.y += 0.5 * (subject.0.y_max - subject.0.y_min + 1.5);
                                result_subject.y += 0.5 * (object.0.y_min - subject.0.y_max - 1.5);
                            }
                        }
                        axis = "y".to_string();
                    }
                }
                collide_aabb::Collision::Inside => {}
            }
        }
        None => {}
    }

    //return (Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), "x".to_string())
    return (result_subject, result_object, axis);
}

fn collide_event_writer(
    mut query: Query<(&mut AABBCollideBox, &GlobalTransform, &Velocity, Entity)>,
    mut events: EventWriter<CollisionEvent>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [(mut staff1_box, staff1_position, staff1_velocity, staff1), (mut staff2_box, staff2_position, staff2_velocity, staff2)],
    ) = combinations.fetch_next()
    {
        let mut subject = (
            &staff1_box,
            staff1_position.clone(),
            staff1_velocity,
            staff1,
        );
        let mut object = (
            &staff2_box,
            staff2_position.clone(),
            staff2_velocity,
            staff2,
        );
        let (subject_position, object_position, collision_axis) =
            collide_check(&subject, &object, &time);

        subject.1.translation = subject_position;
        object.1.translation = object_position;

        if subject.1.translation != staff1_position.translation
            || object.1.translation != staff2_position.translation
        {
            events.send(CollisionEvent {
                subject: subject.3,
                object: object.3,
                subject_transform: subject.1.clone(),
                object_transform: object.1.clone(),
                collision_axis,
            })
        } else {
            staff1_box.collsion_state = false;
            staff2_box.collsion_state = false;
        }
    }
}

fn collision_event_handler(
    mut query: Query<(Entity, &mut Transform)>,
    mut event_handler: EventReader<CollisionEvent>,
) {
    for collision_event in event_handler.iter() {
        //println!("{:?}, {:?}", collision_event.subject, collision_event.object);
        for mut staff in query.iter_mut() {
            if staff.0 == collision_event.subject {
                staff.1.translation = collision_event.subject_transform.translation;
            } else if staff.0 == collision_event.object {
                staff.1.translation = collision_event.object_transform.translation;
            }
        }
    }
}

fn friction(mut query: Query<&mut Velocity, With<board::Board>>, time: Res<Time>) {
    let mut velocity = query.single_mut();
    velocity.x = velocity.x - velocity.x * 0.95 * time.delta_seconds();
    velocity.y = velocity.y - velocity.y * 0.95 * time.delta_seconds();
    if velocity.x * velocity.x < 0.05 {
        velocity.x = 0.0;
    }
    if velocity.y * velocity.y < 0.05 {
        velocity.y = 0.0;
    }
}

fn board_collision(
    mut query: Query<(Entity, &mut Velocity), With<board::Board>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
) {
    for collision_event in collision_event_reader.iter() {
        for mut staff in query.iter_mut() {
            if staff.0 == collision_event.subject {
                if collision_event.collision_axis == "x".to_string() {
                    staff.1.x = 0.0;
                } else if collision_event.collision_axis == "y".to_string() {
                    staff.1.y = 0.0;
                }
            } else if staff.0 == collision_event.object {
                if collision_event.collision_axis == "x".to_string() {
                    staff.1.x = 0.0;
                } else if collision_event.collision_axis == "y".to_string() {
                    staff.1.y = 0.0;
                }
            }
        }
    }
}

fn ball_collision(
    mut query: Query<(&mut Velocity, &GlobalTransform, Entity), With<ball::Ball>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
) {
    for collision_event in collision_event_reader.iter() {
        let mut staff = query.single_mut();
        if staff.2 == collision_event.subject {
            let direction =
                (collision_event.subject_transform.translation - staff.1.translation).truncate();
            let mut k1 =
                direction.x / (direction.x * direction.x + direction.y * direction.y).sqrt();
            let mut k2 =
                direction.y / (direction.x * direction.x + direction.y * direction.y).sqrt();

            if k1 * k1 < 0.08 {
                k1 = 0.3 * k1.signum();
                k2 = 0.9 * k2.signum();
            } else if k2 * k2 < 0.16 {
                k2 = 0.4 * k2.signum();
                k1 = 0.9 * k1.signum();
            }
            if collision_event.collision_axis == "x".to_string() {
                (staff.0.x, staff.0.y) = (k1 * 40.0, -k2 * 40.0);
            } else if collision_event.collision_axis == "y".to_string() {
                (staff.0.x, staff.0.y) = (-k1 * 40.0, k2 * 40.0);
            }
        } else if staff.2 == collision_event.object {
            let direction =
                (collision_event.object_transform.translation - staff.1.translation).truncate();
            let mut k1 =
                direction.x / (direction.x * direction.x + direction.y * direction.y).sqrt();
            let mut k2 =
                direction.y / (direction.x * direction.x + direction.y * direction.y).sqrt();
            if k1 * k1 < 0.08 {
                k1 = 0.3 * k1.signum();
                k2 = 0.9 * k2.signum();
            } else if k2 * k2 < 0.16 {
                k2 = 0.4 * k2.signum();
                k1 = 0.9 * k1.signum();
            }
            if collision_event.collision_axis == "x".to_string() {
                (staff.0.x, staff.0.y) = (k1 * 40.0, -k2 * 40.0);
            } else if collision_event.collision_axis == "y".to_string() {
                (staff.0.x, staff.0.y) = (-k1 * 40.0, k2 * 40.0);
            }
        }
    }
}

pub struct PhysicalPlugin;
impl Plugin for PhysicalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame)
                .label("movement")
                .after("collision")
                .with_system(movement)
                .with_system(friction.after(movement))
                .with_system(collide_box_update)
                .with_system(collide_event_writer.after("collision")),
        );
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame)
                .label("collision")
                .with_system(collision_event_handler)
                .with_system(board_collision)
                .with_system(ball_collision)
                .with_system(collide_box_update),
        );
    }
}
