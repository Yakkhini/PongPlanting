use bevy::{prelude::*, window::PresentMode, sprite::Anchor, input};

#[derive(Component)]
struct Board;

#[derive(Component)]
struct Ball{
    pub speed: Vec2,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 1080.,
            height: 720.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world_system)
        .add_startup_system(set_camera)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_ball)
        .add_system(board_movement)
        .add_system(ball_movement)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .run();
}

fn hello_world_system() {
    println!("hello world");
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_board(mut commands: Commands, assets_server: Res<AssetServer>) { 
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0,200.0)),
            anchor: Anchor::TopCenter,
            ..default()
        },
        texture: assets_server.load("sprites/Sprite-0001.png"),
        ..default()
    })
    .insert(Board)
    .insert_bundle(TransformBundle {
        ..default()
    });
}

fn spawn_ball (mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        texture: assets_server.load("sprites/Ball-texture.png"),
        ..default()
    })
    .insert(Ball {
        speed: Vec2::new(3.0, -3.0)
    })
    .insert_bundle(TransformBundle {
        ..default()
    });
}

fn board_movement (mut query: Query<&mut GlobalTransform, With<Board>>, keyboard_input: Res<Input<input::keyboard::KeyCode>>) {
    
    for mut global_transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) | keyboard_input.pressed(KeyCode::W) {
            global_transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) | keyboard_input.pressed(KeyCode::S) {
            global_transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Left) | keyboard_input.pressed(KeyCode::A) {
            global_transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) | keyboard_input.pressed(KeyCode::D) {
            global_transform.translation.x += 2.;
        }
    }
}

fn ball_movement (mut query: Query<(&mut GlobalTransform, &Ball), With<Ball>>) {
    for (mut global_transform, ball) in query.iter_mut() {
        global_transform.translation.x += ball.speed.x;
        global_transform.translation.y += ball.speed.y;
    }
}