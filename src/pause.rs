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

use crate::appstate;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct PauseButton;

fn pause_enter(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<appstate::AppState>>) {
    if keyboard_input.just_released(KeyCode::Escape) {
        state.push(appstate::AppState::Pause).unwrap();
    }
}

fn pause_exit(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<appstate::AppState>>) {
    if keyboard_input.just_released(KeyCode::Space) {
        state.pop().unwrap();
    }
}

fn pause_setup(mut physics_time: ResMut<PhysicsTime>) {
    physics_time.pause();
}

fn setup_pause_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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
                    "Resume",
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
        .insert(PauseButton)
        .insert(Name::new("Pause Button"));
}

fn pause_button_system(
    mut state: ResMut<State<appstate::AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.pop().unwrap();
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

fn pause_cleanup(
    mut commands: Commands,
    mut physics_time: ResMut<PhysicsTime>,
    query: Query<Entity, With<PauseButton>>,
) {
    let pause_button_entity = query.single();
    commands.entity(pause_button_entity).despawn_recursive();
    physics_time.resume();
}

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame).with_system(pause_enter),
        );
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::Pause)
                .with_system(pause_exit)
                .with_system(pause_button_system),
        );
        app.add_system_set(
            SystemSet::on_enter(appstate::AppState::Pause)
                .with_system(pause_setup)
                .with_system(setup_pause_ui),
        );
        app.add_system_set(
            SystemSet::on_exit(appstate::AppState::Pause).with_system(pause_cleanup),
        );
    }
}
