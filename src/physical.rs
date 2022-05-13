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
    &object: &(&AABBCollideBox, &GlobalTransform, Entity)) -> 
    Vec3 {
        let mut result = subject.1.translation.clone();
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
                        result.x += object.0.x_min - subject.0.x_max;
                    },
                    collide_aabb::Collision::Right => {
                        result.x += object.0.x_max - subject.0.x_min;
                    },
                    collide_aabb::Collision::Top => {
                        result.y += object.0.y_max - subject.0.y_min;
                    },
                    collide_aabb::Collision::Bottom => {
                        result.y += object.0.y_min - subject.0.y_max;
                    },
                    collide_aabb::Collision::Inside => {
                        
                    },
                }
            },
            None => {},
        }

        return result;
}

fn collide_event_writer(
    query: Query<(&AABBCollideBox, &GlobalTransform, Entity)>,
    mut events: EventWriter<CollisionEvent>,
) {
    let mut combinations = query.iter_combinations();
    while let Some([(staff1_box, staff1_position, staff1),
    (staff2_box, staff2_position, staff2)]) = combinations.fetch_next() {
        let mut subject = (staff1_box, staff1_position.clone(), staff1);
        let object = (staff2_box, staff2_position, staff2);
        subject.1.translation = collide_check(&subject, &object).clone();
        
        if subject.1.translation != staff1_position.translation {
            events.send(CollisionEvent {
                subject: subject.2,
                object: object.2,
                subject_transform: subject.1.clone(),
                object_transform: object.1.clone(),
            })
        }
        
    }
}

fn collision_event_handler (mut query: Query<(Entity, &mut Transform)>,
    mut event_handler: EventReader<CollisionEvent>
) {
    for cillision_event in event_handler.iter() {
        println!("{:?}, {:?}", cillision_event.subject, cillision_event.object);
        for mut staff in query.iter_mut() {
            if staff.0 == cillision_event.subject {
                staff.1.translation = cillision_event.subject_transform.translation;
            } else if staff.0 == cillision_event.object {
                staff.1.translation = cillision_event.object_transform.translation;
            }

        }
    }
}