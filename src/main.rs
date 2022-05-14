use bevy::{prelude::*, window::{PresentMode, WindowMode}};

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
        .add_plugin(physical::PhysicalPlugin)
        .add_startup_system(hello_world_system)
        .add_startup_system(set_camera)
        .add_startup_system(board::spawn_board)
        .add_startup_system(ball::spawn_ball)
        .add_startup_system(wall::spawn_bgwall)
        .add_system(board::board_movement)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}