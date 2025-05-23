use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

const FOV: f32 = 66.;
const SPEED: f32 = 5.;
const ROTATIONAL_SPEED: f32 = 1.;
const SCREEN_WIDTH: f32 = 1920.;
const SCREEN_HEIGHT: f32 = 1080.;
const NUM_COLS: usize = 24;
const NUM_ROWS: usize = 24;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    });
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct Gz {}

#[derive(Resource)]
struct Grid([[i32; NUM_ROWS]; NUM_COLS]);

impl Grid {
    fn new() -> Grid {
        Grid([
            [
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            [
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
        ])
    }
}

#[derive(Component)]
struct Me {
    position: Vec2,
    direction: Vec2,
    plane: Vec2,
}

/// Calculates the plane vector given a direction vector and the target field of view.
///
/// This function computes a plane vector perpendicular to the given direction vector,
/// scaled based on the specified field of view.
///
/// # Arguments
///
/// * `dir` - A reference to a 2D vector representing the direction.
/// * `fov` - The field of view in degrees.
///
/// # Returns
///
/// A `Vec2` representing the calculated plane vector.
///
/// # Examples
///
/// ```
/// let direction = Vec2::new(1.0, 0.0);
/// let fov = 60.0;
/// let plane = plane_from_direction(&direction, fov);
/// ```
fn plane_from_direction(dir: &Vec2, fov: f32) -> Vec2 {
    let len_dir = dir.normalize().length();
    let scale_camera_plane = f32::tan(fov.to_radians() / 2.) * len_dir;
    dir.perp() * scale_camera_plane
}

impl Me {
    fn new(x: f32, y: f32) -> Me {
        let position = Vec2::new(x, y);

        // North means bottom of the inline grid
        let direction = Vec2::new(0., -1.);
        let plane = plane_from_direction(&direction, FOV);
        println!("{}", plane);
        Me {
            position,
            direction,
            plane,
        }
    }

    fn rotate(&mut self, rad: f32) {
        let cos: f32 = f32::cos(rad);
        let sin: f32 = f32::sin(rad);
        self.direction = Vec2::new(
            cos * self.direction.x + -sin * self.direction.y,
            sin * self.direction.x + cos * self.direction.y,
        );
        self.plane = Vec2::new(
            cos * self.plane.x + -sin * self.plane.y,
            sin * self.plane.x + cos * self.plane.y,
        );
    }

    fn forward(&mut self, time_delta: f32) {
        let pos_delta = self.direction.normalize_or_zero() * SPEED * time_delta;
        self.position += pos_delta
    }

    fn backward(&mut self, time_delta: f32) {
        let pos_delta = self.direction.normalize_or_zero() * SPEED * time_delta;
        self.position -= pos_delta
    }

    // fn check_if_valid_dest(dest: Vec2) -> bool {
    //     true
    // }
}

fn setup_me(mut commands: Commands) {
    commands.spawn(Me::new(NUM_COLS as f32 / 2., NUM_ROWS as f32 / 2.));
}

struct FloatStepper {
    current: f32,
    step: f32,
    end: f32,
}

impl Iterator for FloatStepper {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

fn float_stepper(start: f32, end: f32, step: f32) -> FloatStepper {
    FloatStepper {
        current: start,
        step,
        end,
    }
}

#[derive(Component)]
struct StatsPane;

fn setup_stats(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Renders player stats to the screen
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let left_column = commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::axes(Val::Px(5.), Val::Px(5.)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        format!("pos:\n"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                    ),
                    TextSection::new(
                        format!("dir:\n"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                    ),
                ]),
                StatsPane,
            ));
        })
        .id();

    commands.entity(left_column);
}

fn update_stats(me: Query<&Me>, mut text_query: Query<&mut Text, With<StatsPane>>) {
    let Ok(me) = me.get_single() else {
        return;
    };

    for mut text in &mut text_query {
        let (current_x, current_y) = me.position.into();
        let (dir_x, dir_y) = me.direction.into();
        text.sections[0].value = format!("pos: [{current_x}, {current_y}]\n");
        text.sections[1].value = format!("dir: [{dir_x}, {dir_y}]");
    }
}

fn handle_movement(
    key: Res<ButtonInput<KeyCode>>,
    mut me: Query<&mut Me>,
    time: Res<Time>,
    grid: Res<Grid>,
) {
    let Ok(mut me) = me.get_single_mut() else {
        return;
    };

    if key.pressed(KeyCode::KeyI) || key.pressed(KeyCode::ArrowUp) {
        me.forward(time.delta_seconds())
    }
    if key.pressed(KeyCode::KeyK) || key.pressed(KeyCode::ArrowDown) {
        me.backward(time.delta_seconds())
    }
    if key.pressed(KeyCode::KeyJ) || key.pressed(KeyCode::ArrowLeft) {
        me.rotate(-ROTATIONAL_SPEED * time.delta_seconds());
    }
    if key.pressed(KeyCode::KeyL) || key.pressed(KeyCode::ArrowRight) {
        me.rotate(ROTATIONAL_SPEED * time.delta_seconds());
    }
}

fn cast_ray(screen_x: f32, me: &Me, grid: &Grid) -> (Vec2, Vec2) {
    // camera_x is also the point on the camera's plane vector where we want to shoot the
    // ray through
    let camera_x = ((screen_x * 2.) / SCREEN_WIDTH) - 1.;
    let ray_direction = me.direction + me.plane * camera_x;
    // position of the ray on the grid
    let mut ray_pos = Vec2::floor(me.position);
    let ray_dist_1_x = f32::abs(ray_direction.length() / ray_direction.x);
    let ray_dist_1_y = f32::abs(ray_direction.length() / ray_direction.y);
    let step_x: i32;
    let step_y: i32;
    let mut dist_x: f32;
    let mut dist_y: f32;
    let mut hit = 0;
    let mut wall_type: i32 = 0;

    if ray_direction.x < 0. {
        step_x = -1;
        // this calculates how far the ray travels when going to the next -column
        dist_x = (me.position.x - ray_pos.x) * ray_dist_1_x;
    } else {
        step_x = 1;
        // this calculates how far the ray travels when going to the next +column
        dist_x = (ray_pos.x + 1.0 - me.position.x) * ray_dist_1_x;
    }
    if ray_direction.y < 0. {
        step_y = -1;
        dist_y = (me.position.y - ray_pos.y) * ray_dist_1_y;
    } else {
        step_y = 1;
        dist_y = (ray_pos.y + 1.0 - me.position.y) * ray_dist_1_y;
    }

    let mut side: i32 = 0;
    while hit == 0 {
        if dist_x < dist_y {
            dist_x += ray_dist_1_x;
            ray_pos.x += step_x as f32;
            side = 0;
        } else {
            dist_y += ray_dist_1_y;
            ray_pos.y += step_y as f32;
            side = 1;
        }
        if grid.0[ray_pos.y as usize][ray_pos.x as usize] > 0 {
            hit = 1;
            wall_type = grid.0[ray_pos.y as usize][ray_pos.x as usize];
        }
    }
    // println!("hit! x: {} y: {} side: {}", ray_pos.x, ray_pos.y, side);
    let mut perp_wall_dist: f32 = 0.;
    if side == 0 {
        perp_wall_dist += dist_x - ray_dist_1_x;
    } else {
        perp_wall_dist += dist_y - ray_dist_1_y;
    };
    let line_height: f32 = SCREEN_HEIGHT / perp_wall_dist;
    let mut line_start = -line_height / 2. + SCREEN_HEIGHT / 2.;
    if line_start < 0. {
        line_start = 0.
    };
    let mut line_end = line_height / 2. + SCREEN_HEIGHT / 2.;
    if line_end > SCREEN_HEIGHT {
        line_end = SCREEN_HEIGHT - 1.
    };
    // println!("start: {}, end: {}", line_start, line_end);

    let mut color: Color;
    match wall_type {
        1 => color = Color::srgb(255. / 255., 190. / 255., 152. / 255.),
        2 => color = Color::srgb(240. / 255., 90. / 255., 126. / 255.),
        3 => color = Color::srgb(18. / 255., 91. / 255., 154. / 255.),
        4 => color = Color::srgb(11. / 255., 132. / 255., 148. / 255.),
        _ => color = Color::srgb(0., 0., 0.),
    }

    if side == 0 {
        color = color.lighter(0.05);
    }
    (
        Vec2::new(camera_x * SCREEN_WIDTH, line_start - SCREEN_HEIGHT / 2.),
        Vec2::new(camera_x * SCREEN_WIDTH, line_end - SCREEN_HEIGHT / 2.),
    )

    // gz.line_2d(
    //     Vec2::new(camera_x * SCREEN_WIDTH, line_start - SCREEN_HEIGHT / 2.),
    //     Vec2::new(camera_x * SCREEN_WIDTH, line_end - SCREEN_HEIGHT / 2.),
    //     color,
    // )
}

fn cast_rays(me: Query<&Me>, grid: Res<Grid>, mut gz: Gizmos<Gz>) {
    let Ok(me) = me.get_single() else { return };
    for x in float_stepper(0., SCREEN_WIDTH, 1.) {
        // camera_x is also the point on the camera's plane vector where we want to shoot the
        // ray through
        let camera_x = ((x * 2.) / SCREEN_WIDTH) - 1.;
        let ray_direction = me.direction + me.plane * camera_x;
        // println!("{}", ray_direction);
        let mut ray_pos = Vec2::floor(me.position);
        let ray_dist_1_x = f32::abs(ray_direction.length() / ray_direction.x);
        let ray_dist_1_y = f32::abs(ray_direction.length() / ray_direction.y);
        let step_x: i32;
        let step_y: i32;
        let mut dist_x: f32;
        let mut dist_y: f32;
        let mut hit = 0;
        let mut wall_type: i32 = 0;

        if ray_direction.x < 0. {
            step_x = -1;
            // this calculates how far the ray travels when going to the next -column
            dist_x = (me.position.x - ray_pos.x) * ray_dist_1_x;
        } else {
            step_x = 1;
            // this calculates how far the ray travels when going to the next +column
            dist_x = (ray_pos.x + 1.0 - me.position.x) * ray_dist_1_x;
        }
        if ray_direction.y < 0. {
            step_y = -1;
            dist_y = (me.position.y - ray_pos.y) * ray_dist_1_y;
        } else {
            step_y = 1;
            dist_y = (ray_pos.y + 1.0 - me.position.y) * ray_dist_1_y;
        }

        let mut side: i32 = 0;
        while hit == 0 {
            if dist_x < dist_y {
                dist_x += ray_dist_1_x;
                ray_pos.x += step_x as f32;
                side = 0;
            } else {
                dist_y += ray_dist_1_y;
                ray_pos.y += step_y as f32;
                side = 1;
            }
            if grid.0[ray_pos.y as usize][ray_pos.x as usize] > 0 {
                hit = 1;
                wall_type = grid.0[ray_pos.y as usize][ray_pos.x as usize];
            }
        }
        // println!("hit! x: {} y: {} side: {}", ray_pos.x, ray_pos.y, side);
        let mut perp_wall_dist: f32 = 0.;
        if side == 0 {
            perp_wall_dist += dist_x - ray_dist_1_x;
        } else {
            perp_wall_dist += dist_y - ray_dist_1_y;
        };
        let line_height: f32 = SCREEN_HEIGHT / perp_wall_dist;
        let mut line_start = -line_height / 2. + SCREEN_HEIGHT / 2.;
        if line_start < 0. {
            line_start = 0.
        };
        let mut line_end = line_height / 2. + SCREEN_HEIGHT / 2.;
        if line_end > SCREEN_HEIGHT {
            line_end = SCREEN_HEIGHT - 1.
        };
        // println!("start: {}, end: {}", line_start, line_end);

        let mut color: Color;
        match wall_type {
            1 => color = Color::srgb(255. / 255., 190. / 255., 152. / 255.),
            2 => color = Color::srgb(240. / 255., 90. / 255., 126. / 255.),
            3 => color = Color::srgb(18. / 255., 91. / 255., 154. / 255.),
            4 => color = Color::srgb(11. / 255., 132. / 255., 148. / 255.),
            _ => color = Color::srgb(0., 0., 0.),
        }

        if side == 0 {
            color = color.lighter(0.05);
        }

        gz.line_2d(
            Vec2::new(camera_x * SCREEN_WIDTH, line_start - SCREEN_HEIGHT / 2.),
            Vec2::new(camera_x * SCREEN_WIDTH, line_end - SCREEN_HEIGHT / 2.),
            color,
        )
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Hello, Raycasting".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .init_gizmo_group::<Gz>()
        .insert_resource(Grid::new())
        .add_systems(Startup, (setup_camera, setup_stats, setup_me))
        //.add_systems(Update, display_grid)
        .add_systems(Update, (handle_movement, update_stats, cast_rays))
        .run();
}
