/*
Copyright (c) 2022 Yakkhini
Planting Pong is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
         http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
*/

use bevy::prelude::*;
use heron::prelude::*;

use crate::{appstate, ball, brick};

pub struct BallBrickCollisionEvent {
    pub brick: Entity,
}

fn detect_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} started to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
            CollisionEvent::Stopped(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} stopped to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
        }
    }
}

fn ball_brick_collision_event_writer(
    mut collide_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<BallBrickCollisionEvent>,
    query_brick: Query<Entity, With<brick::Brick>>,
    query_ball: Query<Entity, With<ball::Ball>>,
) {
    let ball_entity = query_ball.single();
    for event in collide_events.iter() {
        if let CollisionEvent::Stopped(data1, data2) = event {
            for e in query_brick.iter() {
                if (e == data1.rigid_body_entity() && data2.rigid_body_entity() == ball_entity)
                    || (e == data2.rigid_body_entity() && data1.rigid_body_entity() == ball_entity)
                {
                    event_writer.send(BallBrickCollisionEvent { brick: e });
                }
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BallBrickCollisionEvent>();
        app.add_system_set(
            SystemSet::on_enter(appstate::AppState::InGame).with_system(detect_collisions),
        );
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame)
                .with_system(ball_brick_collision_event_writer),
        );
    }
}
