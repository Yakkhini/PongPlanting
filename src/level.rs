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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const LEVEL_MAX: i32 = 20;

#[derive(Component)]
struct LevelNode;

#[derive(Component)]
struct LevelButton {
    level_number: i32,
}

pub struct LevelInfo {
    pub level_number: i32,
}

fn setup_level_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::FlexStart,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::SpaceBetween,
                margin: Rect::all(Val::Auto),
                size: Size {
                    width: Val::Auto,
                    height: Val::Px(1000.0),
                },
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            let mut count = 1;
            loop {
                let mut count_inner = 1;
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            align_items: AlignItems::FlexStart,
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: Rect::all(Val::Auto),
                            size: Size {
                                width: Val::Px(1600.0),
                                height: Val::Auto,
                            },
                            ..default()
                        },
                        color: UiColor(Color::NONE),
                        ..default()
                    })
                    .with_children(|inner_parent| {
                        loop {
                            inner_parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(180.0), Val::Px(65.0)),
                                        // center button
                                        margin: Rect::all(Val::Auto),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: NORMAL_BUTTON.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "level ".to_string() + &count.to_string(),
                                            TextStyle {
                                                font: asset_server.load("fonts/mplus_hzk_12.ttf"),
                                                font_size: 40.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(LevelButton {
                                    level_number: count,
                                })
                                .insert(Name::new("Level Button".to_string() + &count.to_string()));
                            count += 1;
                            if count_inner == 5 {
                                break;
                            }
                            count_inner += 1;
                        }
                    });
                if count >= LEVEL_MAX {
                    break;
                }
            }
        })
        .insert(LevelNode);
}

fn level_button_system(
    mut state: ResMut<State<appstate::AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &LevelButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut level_info: ResMut<LevelInfo>,
) {
    for (interaction, mut color, level_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                level_info.level_number = level_button.level_number;
                state.set(appstate::AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn back_to_menu(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<appstate::AppState>>) {
    if keyboard_input.just_released(KeyCode::Escape) {
        state.set(appstate::AppState::Menu).unwrap();
    }
}

fn level_clean_up(mut commands: Commands, mut query: Query<Entity, With<LevelNode>>) {
    for item in query.iter_mut() {
        commands.entity(item).despawn_recursive()
    }
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelInfo { level_number: 0 });
        app.add_system_set(
            SystemSet::on_enter(appstate::AppState::Level).with_system(setup_level_button),
        );
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::Level)
                .with_system(level_button_system)
                .with_system(back_to_menu),
        );
        app.add_system_set(
            SystemSet::on_exit(appstate::AppState::Level).with_system(level_clean_up),
        );
    }
}
