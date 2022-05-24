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
struct GridLocation {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Brick;

fn create_map(assets_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = assets_server.load("maps/01.ron");

    scene_spawner.spawn_dynamic(scene_handle);
}

fn spawn_bricks(
    assets_server: Res<AssetServer>,
    mut commands: Commands,
    bricks: Query<(&Brick, &GridLocation, Entity), Without<Sprite>>,
) {
    for (_brick, grid_location, e) in bricks.iter() {
        commands.entity(e).insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 18.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    grid_location.x as f32 + 300.0,
                    grid_location.y as f32 + 300.0,
                    2.0,
                ),
                ..default()
            },
            texture: assets_server.load("sprites/brick.png"),
            ..default()
        });
    }
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(appstate::AppState::InGame).with_system(create_map));

        app.add_system_set(
            SystemSet::on_update(appstate::AppState::InGame).with_system(spawn_bricks),
        );
    }
}
