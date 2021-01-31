use bevy::{
    ecs::DynamicBundle,
    prelude::*,
    render::pass::ClearColor,
    //bevy_input::*,
    //window::CursorMoved,
    window::WindowMode,
};

const APP_SIZE: f32 = 800.0;
/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "minesweeper".to_string(),
            width: APP_SIZE,
            height: APP_SIZE,
            vsync: true,
            resizable: true,
            mode: WindowMode::Windowed,
            //WindowMode::Fullscreen { use_size: true },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //.add_resource(Scoreboard { score: 0 })
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup.system())
        .add_system(gen_cell.system())
        .add_system(gen_board.system())
        // .add_system(paddle_movement_system.system())
        // .add_system(ball_collision_system.system())
        // .add_system(ball_movement_system.system())
        // .add_system(scoreboard_system.system())
        .run();
}

#[derive(Bundle)]
struct Cell {
    size: usize,
    color: Color,
    cell_state: CellState,
    //marked: bool,
    revealed: bool,
}

#[derive(Bundle)]
struct Grid {
    rows: usize,
    columns: usize,
}

pub enum CellState {
    Number(u8),
    Mine(bool),
    None,
}

// enum Collider {
//     Solid,
//     Scorable,
//     Paddle,
// }

// #[derive(Default)]
// struct State { // Set up from example
//     mouse_button_event_reader: EventReader<MouseButtonInput>,
//     cursor_moved_event_reader: EventReader<CursorMoved>,
// }

struct MouseLoc(Vec2);

// fn select_character(
//     mut state: ResMut<State>,
//     mouse_pos: ResMut<MouseLoc>,
//     mouse_button_input_events: Res<Events<MouseButtonInput>>,
// ) {
//     for event in state
//         .mouse_button_event_reader
//         .iter(&mouse_button_input_events)
//     {
//         println!("event: {:?} position: {:?}", event, mouse_pos.0);
//     }
// }

// fn mouse_movement_updating_system(
//     mut mouse_pos: ResMut<MouseLoc>,
//     mut state: ResMut<State>,
//     cursor_moved_events: Res<Events<CursorMoved>>,
// ) {
//     for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
//         mouse_pos.0 = event.position;
//     }
// }

// fn main() {
//     App::build()
//         .add_default_plugins()
//         ...
//         .add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
//         .add_system(mouse_movement_updating_system.system());
//         .add_system(position_mouse_click_system.system());
//         ...
//         .run();
// }

fn gen_cell(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let cell_mat = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    println!("Hi");
}

fn gen_board(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // let cell_mat = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    // commands
    // .spawn((Cell{ size: 20.0, position: Vec3::new(10.0,10.0,0.0 )},));
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let rows = 10;
    let columns = 10;
    let cell_spacing = 10.0;
    let cell_size = Vec2::new(50.0, 50.0);
    //let cell_width = columns as f32 * cell_size.x + cell_spacing;
    let cell_material = materials.add(Color::rgb(0.3, 0.3, 0.5).into());
    let mut cell_positions = Vec::new();

    //TODO: figure out spacing
    for row in 0..rows {
        let y_position = row as f32 * (cell_size.y + cell_spacing) ;
        for column in 0..columns {
            let cell_position = Vec3::new(
                column as f32 * (cell_size.x + cell_spacing),
                y_position,
                0.0,
            );
            cell_positions.push(Transform::from_translation(cell_position));
            commands
                .spawn(SpriteBundle {
                    material: cell_material.clone(),
                    transform: Transform::from_translation(cell_position),
                    sprite: Sprite::new(Vec2::new(50.0, 50.0)),
                    ..Default::default()
                })
                .with(Cell {
                    size: 100,
                    color: Color::BLACK,
                    cell_state: CellState::None,
                    revealed: false,
                });
        }
        println!("{:?}", cell_positions);
    }
}

//for row in 0..brick_rows {
//         let y_position = row as f32 * (brick_size.y + brick_spacing);
//         for column in 0..brick_columns {
//             let brick_position = Vec3::new(
//                 column as f32 * (brick_size.x + brick_spacing),
//                 y_position,
//                 0.0,
//             ) + bricks_offset;
//             commands
//                 // brick
//                 .spawn(SpriteBundle {
//                     material: brick_material.clone(),
//                     sprite: Sprite::new(brick_size),
//                     transform: Transform::from_translation(brick_position),
//                     ..Default::default()
//                 })
//                 .with(Collider::Scorable);

//
//     // Add the game's entities to our world
//     commands
//         // cameras
//         .spawn(Camera2dBundle::default())
//         .spawn(CameraUiBundle::default())
//         // paddle
//         .spawn(SpriteBundle {
//             material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
//             transform: Transform::from_xyz(0.0, -215.0, 0.0),
//             sprite: Sprite::new(Vec2::new(120.0, 30.0)),
//             ..Default::default()
//         })
//         .with(Paddle { speed: 500.0 })
//         .with(Collider::Paddle)
//         // ball
//         .spawn(SpriteBundle {
//             material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
//             transform: Transform::from_xyz(0.0, -50.0, 1.0),
//             sprite: Sprite::new(Vec2::new(30.0, 30.0)),
//             ..Default::default()
//         })
//         .with(Ball {
//             velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
//         })
//         // scoreboard
//         .spawn(TextBundle {
//             text: Text {
//                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                 value: "Score:".to_string(),
//                 style: TextStyle {
//                     color: Color::rgb(0.5, 0.5, 1.0),
//                     font_size: 40.0,
//                     ..Default::default()
//                 },
//             },
//             style: Style {
//                 position_type: PositionType::Absolute,
//                 position: Rect {
//                     top: Val::Px(5.0),
//                     left: Val::Px(5.0),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             ..Default::default()
//         });

//     // Add walls
//     let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
//     let wall_thickness = 10.0;
//     let bounds = Vec2::new(900.0, 600.0);

//     commands
//         // left
//         .spawn(SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
//             sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
//             ..Default::default()
//         })
//         .with(Collider::Solid)
//         // right
//         .spawn(SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
//             sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
//             ..Default::default()
//         })
//         .with(Collider::Solid)
//         // bottom
//         .spawn(SpriteBundle {
//             material: wall_material.clone(),
//             transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
//             sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
//             ..Default::default()
//         })
//         .with(Collider::Solid)
//         // top
//         .spawn(SpriteBundle {
//             material: wall_material,
//             transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
//             sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
//             ..Default::default()
//         })
//         .with(Collider::Solid);

//     // Add bricks
//     let brick_rows = 4;
//     let brick_columns = 5;
//     let brick_spacing = 20.0;
//     let brick_size = Vec2::new(150.0, 30.0);
//     let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
//     // center the bricks and move them up a bit
//     let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
//     let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
//     for row in 0..brick_rows {
//         let y_position = row as f32 * (brick_size.y + brick_spacing);
//         for column in 0..brick_columns {
//             let brick_position = Vec3::new(
//                 column as f32 * (brick_size.x + brick_spacing),
//                 y_position,
//                 0.0,
//             ) + bricks_offset;
//             commands
//                 // brick
//                 .spawn(SpriteBundle {
//                     material: brick_material.clone(),
//                     sprite: Sprite::new(brick_size),
//                     transform: Transform::from_translation(brick_position),
//                     ..Default::default()
//                 })
//                 .with(Collider::Scorable);
//         }
//     }
// }

// fn paddle_movement_system(
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Paddle, &mut Transform)>,
// ) {
// for (paddle, mut transform) in query.iter_mut() {
// let mut direction = 0.0;
// if keyboard_input.pressed(KeyCode::Left) {
// direction -= 1.0;
// }

// if keyboard_input.pressed(KeyCode::Right) {
// direction += 1.0;
// }

// let translation = &mut transform.translation;
// // move the paddle horizontally
// translation.x += time.delta_seconds() * direction * paddle.speed;
// // bound the paddle within the walls
// translation.x = translation.x.min(380.0).max(-380.0);
// }
// }

// fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
//     // clamp the timestep to stop the ball from escaping when the game starts
//     let delta_seconds = f32::min(0.2, time.delta_seconds());

//     for (ball, mut transform) in ball_query.iter_mut() {
//         transform.translation += ball.velocity * delta_seconds;
//     }
// }

// fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
//     for mut text in query.iter_mut() {
//         text.value = format!("Score: {}", scoreboard.score);
//     }
// }

// fn ball_collision_system(
//     commands: &mut Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
//     collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
// ) {
// for (mut ball, ball_transform, sprite) in ball_query.iter_mut() {
// let ball_size = sprite.size;
// let velocity = &mut ball.velocity;

// // check collision with walls
// for (collider_entity, collider, transform, sprite) in collider_query.iter() {
// let collision = collide(
// ball_transform.translation,
// ball_size,
// transform.translation,
// sprite.size,
// );
// if let Some(collision) = collision {
// // scorable colliders should be despawned and increment the scoreboard on collision
// if let Collider::Scorable = *collider {
// scoreboard.score += 1;
// commands.despawn(collider_entity);
// }

// // reflect the ball when it collides
// let mut reflect_x = false;
// let mut reflect_y = false;

// // only reflect if the ball's velocity is going in the opposite direction of the collision
// match collision {
// Collision::Left => reflect_x = velocity.x > 0.0,
// Collision::Right => reflect_x = velocity.x < 0.0,
// Collision::Top => reflect_y = velocity.y < 0.0,
// Collision::Bottom => reflect_y = velocity.y > 0.0,
// }

// // reflect velocity on the x-axis if we hit something on the x-axis
// if reflect_x {
// velocity.x = -velocity.x;
// }

// // reflect velocity on the y-axis if we hit something on the y-axis
// if reflect_y {
// velocity.y = -velocity.y;
// }

// // break if this collide is on a solid, otherwise continue check whether a solid is also in collision
// if let Collider::Solid = *collider {
// break;
// }
// }
// }
// }
//}
