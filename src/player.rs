use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.7, 0.3), Vec2::new(30.0, 30.0)),
        Transform::from_xyz(0.0, 0.0, 99.0),
        Player,
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed = 150.0;
    let unnormalized_direction = Vec3::new(
        keyboard_input.pressed(KeyCode::ArrowRight) as i32 as f32
            - keyboard_input.pressed(KeyCode::ArrowLeft) as i32 as f32,
        keyboard_input.pressed(KeyCode::ArrowUp) as i32 as f32
            - keyboard_input.pressed(KeyCode::ArrowDown) as i32 as f32,
        0.0,
    );

    if let Some(normalized_direction) = unnormalized_direction.try_normalize() {
        for mut transform in query.iter_mut() {
            transform.translation += normalized_direction * speed * time.delta_secs();
        }
    }
}
