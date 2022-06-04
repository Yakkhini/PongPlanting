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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GridLocation {
    pub x: i32,
    pub y: i32,
}

fn create_map(assets_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = assets_server.load("scenes/01.scn.ron");

    scene_spawner.spawn_dynamic(scene_handle);
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridLocation>();
        app.add_system_set(SystemSet::on_enter(appstate::AppState::InGame).with_system(create_map));
    }
}
