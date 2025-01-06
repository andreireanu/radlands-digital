use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::prelude::*;

pub const WINDOW_WIDTH: f32 = 3840.;
pub const WINDOW_HEIGHT: f32 = 2160.;

pub fn setup(
    mut commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn board
    commands.spawn((
        Sprite::from_image(asset_server.load("components/board_1400.png")),
        Transform::from_xyz(-300.0, 0.0, -3.0),
    ));

    // Spawn background
    let shape = shapes::Rectangle {
        extents: Vec2::new(700.0, WINDOW_HEIGHT), // Non-zero dimensions
        origin: RectangleOrigin::Center,          // Center the rectangle
        radii: Some(BorderRadii::single(10.0)),   // Rounded corners
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_xyz(700.0, 0.0, -2.0), // Center position
            ..default()
        },
        Fill::color(Color::srgba_u8(61, 71, 133, 255)), // Red color
    ));

    // Spawn camera
    commands.spawn(Camera2d {});
}
