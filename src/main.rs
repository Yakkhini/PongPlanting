use bevy::{prelude::*, window::{PresentMode, WindowMode}};

mod board;
mod ball;
mod bump;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Planting Pong".to_string(),
            width: 1090.,
            height: 1080.,
            present_mode: PresentMode::Mailbox,
            resizable: true,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world_system)
        .add_startup_system(set_camera)
        .add_startup_system(board::spawn_board)
        .add_startup_system(ball::spawn_ball)
        .add_event::<bump::Bump>()
        .add_system(board::board_movement)
        .add_system(ball::ball_movement)
        .add_system(bump::bump)
        .add_system(bump::bump_check)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}