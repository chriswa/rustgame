use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.7, 0.3), Vec2::new(30.0, 30.0)),
        Player,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 150.0;
    let raw_direction = Vec3::new(
        keyboard_input.pressed(KeyCode::KeyD) as i32 as f32
            - keyboard_input.pressed(KeyCode::KeyA) as i32 as f32,
        keyboard_input.pressed(KeyCode::KeyW) as i32 as f32
            - keyboard_input.pressed(KeyCode::KeyS) as i32 as f32,
        0.0,
    );
    if let Some(normalized_direction) = raw_direction.try_normalize() {
        for mut transform in query.iter_mut() {
            transform.translation += normalized_direction * speed * time.delta_secs();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Infinite RPG".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_movement)
        .run();
}
