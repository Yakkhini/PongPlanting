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

use crate::{appstate, collision};

#[derive(Component)]
pub struct ScoreText;

struct Score {
    score: i32,
    step: i32,
}

impl Default for Score {
    fn default() -> Score {
        Score { score: 0, step: 1 }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexStart,
                position: Rect {
                    left: Val::Px(1500.0),
                    top: Val::Px(60.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/mplus_hzk_12.ttf"),
                    font_size: 70.0,
                    color: Color::MAROON,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
            ),
            ..default()
        })
        .insert(ScoreText)
        .insert(Name::new("Score Text"));
}

fn score_update(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let score_number = score.score.to_string();
    query.single_mut().sections[0].value = "Score: ".to_string() + &score_number;
}

fn score_count(
    mut ball_brick_events: EventReader<collision::BallBrickCollisionEvent>,
    mut ball_board_events: EventReader<collision::BallBoardCollisionEvent>,
    mut score: ResMut<Score>,
) {
    for _event in ball_brick_events.iter() {
        score.score = score.score + score.step;
        score.step += 1;
    }

    for _event in ball_board_events.iter() {
        score.step = 1;
    }
}

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_system_set(SystemSet::on_enter(appstate::AppState::InGame).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame)
                .with_system(score_update)
                .with_system(score_count),
        );
    }
}
