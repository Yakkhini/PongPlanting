use bevy::{prelude::*, window::PresentMode, sprite::Anchor, input};

#[derive(Component)]
struct Board;

#[derive(Component)]
struct Ball{
    speed: Vec2,
}

struct Bump{
    acceleration: Vec2
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
        .add_event::<Bump>()
        .add_system(board_movement)
        .add_system(ball_movement)
        .add_system(bump)
        .add_system(bump_check)
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
        speed: Vec2::new(0.0, -3.0)
    })
    .insert_bundle(TransformBundle {
        global: GlobalTransform {
            translation: Vec3::new(10.0, 10.0, 0.0),
            ..default()
        },
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

fn bump(
    mut bump_event_reader: EventReader<Bump>,
    mut query: Query<&mut Ball>
) {
    for bump_event in bump_event_reader.iter() {
        for mut ball in query.iter_mut() {
            ball.speed.x = bump_event.acceleration.x;
            ball.speed.y = bump_event.acceleration.y;
        }
    }
}

fn bump_check (
    query_board: Query<&GlobalTransform, With<Board>>,
    query_ball: Query<&GlobalTransform, With<Ball>>,
    mut bump_event_writer: EventWriter<Bump>
) {
    for global_transform_board in query_board.iter() {
        for global_transform_ball in query_ball.iter() {
            if global_transform_ball.translation.x > 
                global_transform_board.translation.x - 10.0 &&
                global_transform_ball.translation.x < 
                global_transform_board.translation.x + 10.0 &&
                global_transform_ball.translation.y > global_transform_board.translation.y - 50.0 &&
                global_transform_ball.translation.y < global_transform_board.translation.y - 40.0 {
                bump_event_writer.send(Bump {
                    acceleration: Vec2::new(0.0, 1.0)
                });
            }
        }
    }
}