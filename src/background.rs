use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// use bevy_prototype_lyon::prelude::*;

pub const BOARD_WIDTH: f32 = 2844.0;
pub const BOARD_HEIGHT: f32 = 2048.0;
pub const BOARD_RATIO: f32 = BOARD_WIDTH / BOARD_HEIGHT;
pub const WINDOW_WIDTH: f32 = 2300.0;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH / BOARD_RATIO;

pub fn setup(
    mut commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn board
    commands.spawn((
        Sprite::from_image(asset_server.load("components/board.png")),
        Transform {
            translation: Vec3::new(0.0, 0.0, -2.0), // Set Z position
            scale: Vec3::new(
                WINDOW_WIDTH / BOARD_WIDTH,
                WINDOW_HEIGHT / BOARD_HEIGHT,
                1.0,
            ),
            ..Default::default()
        },
    ));

    // Spawn camera
    commands.spawn(Camera2d {});
}
