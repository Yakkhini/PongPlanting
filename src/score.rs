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

use crate::appstate;

#[derive(Component)]
struct ScoreText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexStart,
                position: Rect {
                    left: Val::Px(1600.0),
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

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(appstate::AppState::InGame).with_system(setup));
    }
}
