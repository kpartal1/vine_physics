use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

const NUM_SIDES: usize = 100;
const DELTA: f32 = 2. * PI / NUM_SIDES as f32;
const WINDOW_WIDTH: f32 = 1080. / 2.;
const WINDOW_HEIGHT: f32 = 1920. / 2.;
const BIG_RADIUS: f32 = (WINDOW_WIDTH / 2.) * 0.9;
const SQUARE_SIZE: f32 = BIG_RADIUS * 0.15;

#[derive(Component, Debug)]
struct BigRadius(f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());
    let mut window = windows.single_mut();
    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT);

    let restitution = Restitution::coefficient(1.);

    // Big Circle
    let mut vertices = Vec::new();
    let r = BIG_RADIUS;
    let mut theta = 0.;
    while theta < 2. * PI {
        let x = r * f32::cos(theta);
        let y = r * f32::sin(theta);
        vertices.push(Vec2::new(x, y));
        theta += DELTA;
    }
    let mut indices = (1..vertices.len() as u32)
        .map(|i| [i - 1, i])
        .collect::<Vec<_>>();
    indices.push([(vertices.len() - 1) as u32, 0]);
    commands.spawn((
        RigidBody::Fixed,
        Collider::polyline(vertices, Some(indices)),
        restitution,
        BigRadius(BIG_RADIUS),
    ));

    // Small Circles
    let collider_mprops = ColliderMassProperties::Density(0.1);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/vine_logo.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SQUARE_SIZE * 2.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 150., 3.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(SQUARE_SIZE, SQUARE_SIZE),
        restitution,
        collider_mprops,
        ActiveEvents::COLLISION_EVENTS,
        Ccd::enabled(),
    ));

    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: Mesh2dHandle(meshes.add(Circle::new(50.))),
    //         material: materials.add(Color::VIOLET),
    //         transform: Transform::from_xyz(200., 250., 2.),
    //         ..default()
    //     },
    //     RigidBody::Dynamic,
    //     Collider::ball(50.),
    //     restitution,
    //     collider_mprops,
    //     // Ccd::enabled(),
    // ));
}

const SHRINK_SPEED: f32 = 8.;

fn shrink_big_circle(
    mut gizmos: Gizmos,
    mut big_circle_query: Query<(&mut Collider, &mut BigRadius), With<BigRadius>>,
    time: Res<Time>,
) {
    let (mut collider, mut radius) = big_circle_query.single_mut();
    gizmos
        .circle_2d(Vec2::ZERO, radius.0, Color::WHITE)
        .segments(NUM_SIDES);
    let mut vertices = Vec::new();
    let r = radius.0;
    let mut theta = 0.;
    while theta < 2. * PI {
        let x = r * f32::cos(theta);
        let y = r * f32::sin(theta);
        vertices.push(Vec2::new(x, y));
        theta += DELTA;
    }
    let mut indices = (1..vertices.len() as u32)
        .map(|i| [i - 1, i])
        .collect::<Vec<_>>();
    indices.push([(vertices.len() - 1) as u32, 0]);
    *collider = Collider::polyline(vertices, Some(indices));
    radius.0 -= SHRINK_SPEED * time.delta_seconds();
}

const IMAGES: [&str; 10] = [
    "big_spoon",
    "cat",
    "dude",
    "emoji",
    "kirby_squid",
    "omg_cat",
    "omg",
    "spunchbop",
    "the_rock",
    "tiki",
];

const SOUNDS: [&str; 7] = [
    "bruh",
    "deez_nuts",
    "OHHHHH",
    "ohmygod",
    "phone_linging",
    "quandale_dingle",
    "vine_boom",
];

fn handle_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut collider_query: Query<&mut Handle<Image>, With<Collider>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(_, e2, _) = collision_event {
            if let Ok(mut image_handle) = collider_query.get_mut(*e2) {
                *image_handle = asset_server.load(
                    "textures/".to_owned()
                        + IMAGES[rand::random::<usize>() % IMAGES.len()]
                        + ".png",
                );
                commands.spawn(AudioBundle {
                    source: asset_server.load(
                        "sounds/".to_owned()
                            + SOUNDS[rand::random::<usize>() % SOUNDS.len()]
                            + ".ogg",
                    ),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(400.))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, (shrink_big_circle, handle_collision))
        .run();
}
