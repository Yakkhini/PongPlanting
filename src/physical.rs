use bevy::{prelude::*, sprite::collide_aabb};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, PartialEq)]
pub struct AABBCollideBox {
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
    subject_axis: String,
    object_axis: String,
}

pub struct PhysicalPlugin;
impl Plugin for PhysicalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_system(movement);
        app.add_system(collide_event_writer);
        app.add_system(collision_event_handler);
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
    mut query_box: Query<(&mut AABBCollideBox, &GlobalTransform)>
) {
    for (mut collide_box,
        global_transform) in query_box.iter_mut() {
        let (translation, height, width) = (global_transform.translation, collide_box.height, collide_box.width); 
        collide_box.x_min = translation.x - width * 0.5;
        collide_box.x_max = translation.x + width * 0.5;
        collide_box.y_min = translation.y - height * 0.5;
        collide_box.y_max = translation.y + height * 0.5;
        //println!("{:?}, {:?}", collide_box, translation);
        
    }
}

fn collide_check(&subject: &(&AABBCollideBox, GlobalTransform, Entity),
    &object: &(&AABBCollideBox, GlobalTransform, Entity)) -> 
    (Vec3, String) {
        let mut result = subject.1.translation.clone();
        let mut axis = String::new();
        let check_result = collide_aabb::collide(
            subject.1.translation,
            Vec2::new(subject.0.width, subject.0.height),
            object.1.translation,
            Vec2::new(object.0.width, object.0.height)
        );

        match check_result {
            Some(_) => {
                let check_result = check_result.unwrap();
                match check_result {
                    collide_aabb::Collision::Left => {
                        if subject.0.platform == false {
                            result.x += 0.5 * (object.0.x_min - subject.0.x_max);
                            axis = "x".to_string();
                        }
                    },
                    collide_aabb::Collision::Right => {
                        if subject.0.platform == false {
                            result.x += 0.5 * (object.0.x_max - subject.0.x_min);
                            axis = "x".to_string();
                        }
                    },
                    collide_aabb::Collision::Top => {
                        if subject.0.platform == false {
                            result.y += 0.5 * (object.0.y_max - subject.0.y_min);
                            axis = "y".to_string();
                        }
                    },
                    collide_aabb::Collision::Bottom => {
                        if subject.0.platform == false {
                            result.y += 0.5 * (object.0.y_min - subject.0.y_max);
                            axis = "y".to_string();
                        }
                    },
                    collide_aabb::Collision::Inside => {
                        
                    },
                }
            },
            None => {},
        }

        return (result, axis);
}

fn collide_event_writer(
    query: Query<(&AABBCollideBox, &GlobalTransform, Entity)>,
    mut events: EventWriter<CollisionEvent>,
) {
    let mut combinations = query.iter_combinations();
    while let Some([(staff1_box, staff1_position, staff1),
    (staff2_box, staff2_position, staff2)]) = combinations.fetch_next() {
        let mut subject = (staff1_box, staff1_position.clone(), staff1);
        let mut object = (staff2_box, staff2_position.clone(), staff2);
        let (subject_position, subject_axis) = collide_check(&subject, &object);
        let (objrct_position, object_axis) = collide_check(&object, &subject);

        subject.1.translation = subject_position;
        object.1.translation = objrct_position;
        
        if subject.1.translation != staff1_position.translation ||
        object.1.translation != staff2_position.translation {
            events.send(CollisionEvent {
                subject: subject.2,
                object: object.2,
                subject_transform: subject.1.clone(),
                object_transform: object.1.clone(),
                subject_axis,
                object_axis,
            })
        }
        
    }
}

fn collision_event_handler (mut query: Query<(Entity, &mut Transform, &mut Velocity)>,
    mut event_handler: EventReader<CollisionEvent>
) {
    for cillision_event in event_handler.iter() {
        println!("{:?}, {:?}", cillision_event.subject, cillision_event.object);
        for mut staff in query.iter_mut() {
            if staff.0 == cillision_event.subject {
                if cillision_event.subject_axis == "x".to_string() {
                    staff.2.x = 0.0;
                } else if cillision_event.subject_axis == "y".to_string() {
                    staff.2.y = 0.0;
                }
                staff.1.translation = cillision_event.subject_transform.translation;
            } else if staff.0 == cillision_event.object {
                if cillision_event.object_axis == "x".to_string() {
                    staff.2.x = 0.0;
                } else if cillision_event.object_axis == "y".to_string() {
                    staff.2.y = 0.0;
                }
                staff.1.translation = cillision_event.object_transform.translation;
            }

        }
    }
}