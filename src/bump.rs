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

use bevy::prelude::*;

use crate::{ball, board, wall::BackGroundWall};

pub struct Bump {
    acceleration: Vec2,
}

pub fn bump(mut bump_event_reader: EventReader<Bump>, mut query: Query<&mut ball::Ball>) {
    for bump_event in bump_event_reader.iter() {
        for mut ball in query.iter_mut() {
            ball.speed.x += bump_event.acceleration.x;
            ball.speed.y += bump_event.acceleration.y;
        }
    }
}

pub fn bump_check(
    query_board: Query<&GlobalTransform, With<board::Board>>,
    query_ball: Query<&GlobalTransform, With<ball::Ball>>,
    query_wall: Query<&BackGroundWall>,
    mut bump_event_writer: EventWriter<Bump>,
) {
    for global_transform_board in query_board.iter() {
        for global_transform_ball in query_ball.iter() {
            if global_transform_ball.translation.x > global_transform_board.translation.x - 10.0
                && global_transform_ball.translation.x < global_transform_board.translation.x + 10.0
                && global_transform_ball.translation.y > global_transform_board.translation.y - 50.0
                && global_transform_ball.translation.y < global_transform_board.translation.y - 40.0
            {
                bump_event_writer.send(Bump {
                    acceleration: Vec2::new(0.0, 3.0),
                });
            }

            let bgwall = query_wall.single();
            if bgwall.exclude_check(&global_transform_ball.translation) {
                bump_event_writer.send(Bump {
                    acceleration: Vec2::new(0.0, -3.0),
                });
            }
            if bgwall.exclude_check(&global_transform_board.translation) {
                bump_event_writer.send(Bump {
                    acceleration: Vec2::new(0.0, -3.0),
                });
            }
        }
    }
}
