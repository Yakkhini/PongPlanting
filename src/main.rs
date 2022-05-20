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

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod appstate;
mod background;
mod ball;
mod board;
mod physical;
mod wall;

pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planting Pong".to_string(),
            width: 1080.0 * RESOLUTION,
            height: 1080.0,
            position: Some(Vec2::new(0.0, 0.0)),
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::Mailbox,
            #[doc(alias = "vsync")]
            resizable: false,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(appstate::StatesPlugin)
        .add_plugin(background::BackgroundPlugin {})
        .add_plugin(physical::PhysicalPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(ball::BallPlugin)
        .add_plugin(wall::WallPlugin)
        .add_startup_system(hello_world_system)
        .add_startup_system(set_camera)
        .run();
}

fn hello_world_system() {
    println!("hello world");
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
