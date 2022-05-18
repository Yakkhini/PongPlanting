use bevy::{prelude::*, window::{PresentMode, WindowMode}};

mod background;
mod appstate;
mod physical;
mod board;
mod ball;
mod wall;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planting Pong".to_string(),
            width: 1980.,
            height: 1080.,
            position: Some(Vec2::new(0.0, 0.0)),
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::Mailbox,
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