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

use crate::{ball, board, brick, score, wall};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Level,
    InGame,
    Pause,
}

pub struct GameBackToMenuEvent;

fn game_clean_up(
    mut commands: Commands,
    query_ball: Query<Entity, With<ball::Ball>>,
    query_board: Query<Entity, With<board::Board>>,
    query_brick: Query<Entity, With<brick::Brick>>,
    query_ui: Query<Entity, With<score::ScoreText>>,
    query_wall: Query<Entity, With<wall::Wall>>,
    mut score: ResMut<score::Score>,
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

    for e in query_wall.iter() {
        commands.entity(e).despawn_recursive();
    }

    score.score = 0;
    score.step = 1;
}

fn game_back_to_menu_event_handler(
    mut events: EventReader<GameBackToMenuEvent>,
    mut state: ResMut<State<AppState>>,
) {
    for _event in events.iter() {
        state.set(AppState::Menu).unwrap();
    }
}

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameBackToMenuEvent>();
        app.add_state(AppState::Menu);
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(game_back_to_menu_event_handler),
        );
        app.add_system_set(SystemSet::on_exit(AppState::InGame).with_system(game_clean_up));
    }
}
