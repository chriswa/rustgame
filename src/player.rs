use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct MainCameraMarker;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct CameraTarget(pub Vec3);

#[derive(Resource)]
pub struct CameraFollowConfig {
    pub lerp_factor: f32,
}

impl Default for CameraFollowConfig {
    fn default() -> Self {
        Self { lerp_factor: 5.0 }
    }
}

fn is_pressed_any(input: &ButtonInput<KeyCode>, keys: &[KeyCode]) -> f32 {
    keys.iter().any(|&key| input.pressed(key)) as i32 as f32
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MainCameraMarker,
        CameraTarget(Vec3::ZERO),
    ));
    commands.insert_resource(CameraFollowConfig::default());

    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.7, 0.3), Vec2::new(30.0, 30.0)),
        Transform::from_xyz(0.0, 0.0, 99.0),
        PlayerMarker,
        MovementSpeed(150.0),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &MovementSpeed), With<PlayerMarker>>,
    time: Res<Time>,
) {
    let unnormalized_direction = Vec3::new(
        is_pressed_any(&keyboard_input, &[KeyCode::ArrowRight, KeyCode::KeyD])
            - is_pressed_any(&keyboard_input, &[KeyCode::ArrowLeft, KeyCode::KeyA]),
        is_pressed_any(&keyboard_input, &[KeyCode::ArrowUp, KeyCode::KeyW])
            - is_pressed_any(&keyboard_input, &[KeyCode::ArrowDown, KeyCode::KeyS]),
        0.0,
    );

    if let Some(normalized_direction) = unnormalized_direction.try_normalize() {
        for (mut transform, speed) in query.iter_mut() {
            transform.translation += normalized_direction * speed.0 * time.delta_secs();
        }
    }
}

pub fn update_camera_target(
    player_query: Query<&Transform, With<PlayerMarker>>,
    mut camera_query: Query<&mut CameraTarget, With<MainCameraMarker>>,
) -> Result {
    camera_query.single_mut()?.0 = player_query.single()?.translation;
    Ok(())
}

pub fn update_camera_transform(
    mut camera_query: Query<(&mut Transform, &CameraTarget), With<MainCameraMarker>>,
    time: Res<Time>,
    config: Res<CameraFollowConfig>,
) -> Result {
    let (mut camera_transform, camera_target) = camera_query.single_mut()?;
    let lerp_factor = config.lerp_factor * time.delta_secs();
    camera_transform.translation = camera_transform.translation.lerp(
        Vec3::new(
            camera_target.0.x,
            camera_target.0.y,
            camera_transform.translation.z,
        ),
        lerp_factor,
    );
    Ok(())
}
