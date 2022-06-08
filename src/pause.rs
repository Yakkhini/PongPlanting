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

fn pause_resume(mut physics_time: ResMut<PhysicsTime>) {
    physics_time.resume();
}

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame).with_system(pause_enter),
        );
        app.add_system_set(SystemSet::on_update(appstate::AppState::Pause).with_system(pause_exit));
        app.add_system_set(SystemSet::on_enter(appstate::AppState::Pause).with_system(pause_setup));
        app.add_system_set(SystemSet::on_exit(appstate::AppState::Pause).with_system(pause_resume));
    }
}
