use core::f32;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SOME_COLOR: Color = Color::srgb(0.99, 0.24, 0.71);
const SPEED: f32 = 200.;
const CIRCLE_DIAMETER: f32 = 25.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Direction(Vec2);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let initial_location = Vec3::new(0., 0., 1.);

    commands.spawn((
        Player,
        Direction(Vec2::new(0., 1.)),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(SOME_COLOR),
            transform: Transform::from_translation(initial_location)
                .with_scale(Vec2::splat(CIRCLE_DIAMETER).extend(1.0)),
            ..default()
        },
    ));
}

fn degree_to_randian(deg: i32) -> f32 {
    deg as f32 * (f32::consts::PI / 180.)
}

fn rotate_vector(vec: Vec2, deg: i32) -> Vec2 {
    let rad = degree_to_randian(deg);
    let cos: f32 = f32::cos(rad);
    let sin: f32 = f32::sin(rad);
    Vec2::new(cos * vec.x + -sin * vec.y, sin * vec.x + cos * vec.y)
}

fn move_player(
    time: Res<Time>,
    key: Res<ButtonInput<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    mut direction: Query<&mut Direction>,
) {
    let Ok(mut player_transform) = player_transform.get_single_mut() else {
        return;
    };

    let Ok(mut direction) = direction.get_single_mut() else {
        return;
    };

    if key.just_pressed(KeyCode::ArrowLeft) {
        direction.0 = rotate_vector(direction.0, 45);
    };
    if key.just_pressed(KeyCode::ArrowRight) {
        direction.0 = rotate_vector(direction.0, -45);
    };
    if key.pressed(KeyCode::KeyW) || key.pressed(KeyCode::ArrowUp) {
        let move_delta = direction.0.normalize_or_zero() * SPEED * time.delta_seconds();
        player_transform.translation += move_delta.extend(0.);
        println!("New Position: {}", player_transform.translation);
        return;
    };
}

#[derive(Resource)]
struct FPSCounter(usize);

fn display_fps(mut fps_counter: Res<FPSCounter>) {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FPSCounter(0))
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, move_player)
        // .add_systems(Update, (move_circle))
        .run();
}
