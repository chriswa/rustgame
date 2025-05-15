use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

mod player;
mod tilemap;
mod tileset;

use tilemap::Tilemap;
use tileset::Tileset;

struct SamplePlugin;
impl Plugin for SamplePlugin {
    fn build(&self, _app: &mut App) {}
}

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
        .add_plugins(SamplePlugin)
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(
            Startup,
            (
                setup_tileset,
                setup_tilemap.after(setup_tileset),
                player::setup,
            ),
        )
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

fn setup_tileset(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create the tileset
    let tileset = Tileset::new(&mut images);
    commands.insert_resource(tileset);
}

fn setup_tilemap(mut commands: Commands, tileset: Res<Tileset>) {
    // Create a tilemap using the tileset
    let map_size = TilemapSize { x: 32, y: 32 };
    let texture_indices = (0..(map_size.x * map_size.y)).map(|i| (i % 3) as u32);
    let transform = Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(5.0));

    let _tilemap = Tilemap::new(
        &mut commands,
        tileset.into_inner(),
        map_size,
        texture_indices,
        transform,
    );
}
