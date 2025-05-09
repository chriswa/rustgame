use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

mod player;
mod tilemap;

fn get_window_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "rustgame".into(),
            resolution: (800., 600.).into(),
            ..default()
        }),
        ..default()
    }
}

fn main() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);
        std::process::exit(1); // TODO: figure out how to exit after Bevy finishes printing unwind messages
    }));

    App::new()
        .add_plugins(DefaultPlugins.set(get_window_config()))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(TilemapPlugin)
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, (player::setup, tilemap::setup))
        .add_systems(
            FixedUpdate,
            (
                player::player_movement,
                player::update_camera_target,
                player::update_camera_transform,
            ),
        )
        .run();
}
