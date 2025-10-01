use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, linear_move_with_easing)
        .run();
}

const TITLE: &str = "Linear Path + Bezier Easing";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Component)]
struct EasedPlatform {
    start: Vec2,
    end: Vec2,
    t: f32,
    speed: f32,
    forward: bool,
    easing: CubicEasing,
}

/// A cubic-bezier easing curve (same parameters as CSS: (x1, y1, x2, y2))
#[derive(Clone, Copy)]
struct CubicEasing {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl CubicEasing {
    fn ease(&self, t: f32) -> f32 {
        // standard cubic-bezier time easing: find y for given t
        // here we solve for x(t) = input, y(t) = output
        cubic_bezier_y(t, self.x1, self.y1, self.x2, self.y2)
    }
}

/// Cubic-bezier evaluation for easing curves
fn cubic_bezier_y(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // De Casteljau algorithm for y(t)
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    // cubic bezier along y
    uuu * 0.0 +
    3.0 * uu * t * y1 +
    3.0 * u * tt * y2 +
    ttt * 1.0
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.6, 0.6),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
        EasedPlatform {
            start: Vec2::new(-400.0, 0.0),
            end: Vec2::new(200.0, 200.0),
            t: 0.0,
            speed: 2.0, // seconds to traverse 0→1
            forward: true,
            easing: CubicEasing {
                x1: 0.42,
                y1: 0.0,
                x2: 0.58,
                y2: 1.0,
            },
        },
    ));
}

/// Animate t linearly over time, but apply cubic-bezier easing to movement
fn linear_move_with_easing(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut EasedPlatform)>,
) {
    for (mut transform, mut platform) in &mut q {
        let dt = time.delta_secs() / platform.speed;

        // Linear t progression with ping-pong
        if platform.forward {
            platform.t += dt;
            if platform.t >= 1.0 {
                platform.t = 1.0;
                platform.forward = false;
            }
        } else {
            platform.t -= dt;
            if platform.t <= 0.0 {
                platform.t = 0.0;
                platform.forward = true;
            }
        }

        // Apply easing curve to t
        let eased_t = platform.easing.ease(platform.t);

        // Interpolate linearly between start and end using eased_t
        let pos = platform.start.lerp(platform.end, eased_t);
        transform.translation = pos.extend(0.0);
    }
}
