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
        .add_systems(Update, platform_movement)
        .run();
}


const MAX_SPEED: f32 = 500.0;

const TITLE: &str = "platform?";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Component)]
struct Platform;

#[derive(Bundle)]
struct PlatformBundle{
    direction: Direction,
    velocity: Velocity,
    curr: Curr,
    start: Start,
    end: End,
    speed: Speed,
    platform:Platform,
    sprite: Sprite,
}

#[derive(Component, Deref, DerefMut)]
struct Curr (Vec2);

#[derive(Component, Deref, DerefMut)]
struct Start (Vec2);

#[derive(Component, Deref, DerefMut)]
struct End (Vec2);

#[derive(Component, Deref, DerefMut)]
struct Speed (f32);

#[derive(Component, Deref, DerefMut)]
struct Direction (bool);

#[derive(Component, Deref, DerefMut)]
struct Velocity (Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        PlatformBundle{direction: Direction(false), 
            velocity: Velocity(Vec2::splat(0.)),
            curr: Curr(Vec2::new(5., 5.)),
            start: Start(Vec2::new(5.,5.)), end: End(Vec2::new(-300., -300.)),
            speed: Speed(1000.0), platform: Platform,    
            sprite: Sprite{ color: Color::srgb(0.8, 0.6, 0.6), 
                custom_size: Some(Vec2::new(200.0, 20.0)),
                ..default()
            }
        },
        Transform {
                translation: Vec3::new(5., 5.0, 0.0),
                ..default()
        },
    ));
}

fn platform_movement(
    time: Res<Time>,
    mut platforms: Query<(&mut Transform, &mut Velocity,  &mut Curr, &End, &Start, &Speed, &mut Direction), With<Platform>>,
){
    for (mut transform, mut velocity, mut curr, end, start, speed, mut direction) in platforms.iter_mut() {
        let mut dir = Vec2::ZERO;

        let deltat = time.delta_secs();
        let accel = **speed * deltat;


        if **direction{
            dir.x = end.x - curr.x;
            dir.y= end.y - curr.y;
        } else{
            dir.x = start.x - curr.x;
            dir.y = start.y - curr.y;
        }

        **velocity = if dir.length() > 0. {
            (**velocity + (dir.normalize_or_zero() * accel)).clamp_length_max(MAX_SPEED)
        } else if velocity.length() > accel {
            **velocity + (velocity.normalize_or_zero() * -accel)
        } else {
            Vec2::ZERO
        };

        let change = **velocity * deltat;

        transform.translation = transform.translation + change.extend(0.);
        info!(curr.x);
        info!(change.x);
        if **direction{
            info!("true");
        }else{
            info!(false);
        }
        curr.x = curr.x + change.x;
        curr.y = curr.y + change.y;


        if **direction{
            if end.x > start.x{
                if end.y > start.y{
                    if curr.x >= end.x && curr.y >=end.y{
                    **direction = false;
                    **velocity = Vec2::ZERO
                    }
                } else{
                    if curr.x >= end.x && curr.y <=end.y{
                    **direction = false;
                    **velocity = Vec2::ZERO
                    }
                }
            }
            else{
                if end.y > start.y{
                    if curr.x <= end.x && curr.y >=end.y{
                    **direction = false;
                    **velocity = Vec2::ZERO
                    }
                } else{
                    if curr.x <= end.x && curr.y <=end.y{
                    **direction = false;
                    **velocity = Vec2::ZERO
                    }
                }
            }
        } else{
            if start.x > end.x{
                if start.y > end.y{
                    if curr.x >= start.x && curr.y >=start.y{
                        **direction = true;
                    }
                } else{
                    if curr.x >= start.x && curr.y <=start.y{
                        **direction = true;
                    }
                }
            }
            else{
                if start.y > end.y{
                    if curr.x <= start.x && curr.y >=start.y{
                    **direction = true;
                    }
                } else{
                    if curr.x <= start.x && curr.y <=start.y{
                    **direction = true;
                    }
                }
            }
        }
    }
}