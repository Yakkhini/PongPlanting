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

use crate::{board, ball, brick, score};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Level,
    InGame,
    Pause,
}

fn game_clean_up(
    mut commands: Commands,
    query_ball: Query<Entity, With<ball::Ball>>,
    query_board: Query<Entity, With<board::Board>>,
    query_brick: Query<Entity, With<brick::Brick>>,
    query_ui: Query<Entity,With<score::ScoreText>>,
) {
    let ball_entity = query_ball.single();
    let board_entity = query_board.single();
    let ui_entity = query_ui.single();
    commands.entity(ball_entity).despawn();
    commands.entity(board_entity).despawn();
    commands.entity(ui_entity).despawn_recursive();

    for e in query_brick.iter() {
        commands.entity(e).despawn_recursive();
    }

}

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu);
        app.add_system_set(SystemSet::on_exit(AppState::InGame).with_system(game_clean_up));
    }
}
