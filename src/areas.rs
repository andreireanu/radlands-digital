use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const ANIMATION_DURATION: f32 = 0.2;

pub fn animate_glow(time: Res<Time>, mut query: Query<(&mut Stroke, &mut Glow)>) {
    for (mut stroke, _) in query.iter_mut() {
        // Get the elapsed seconds since the app started
        let elapsed = time.elapsed_secs();

        let anim_value = (elapsed / ANIMATION_DURATION).sin(); // Oscillator
        let anim_value_normalized = (anim_value + 1.0) * 0.5; // Normalize to [0, 1]
        let normalized_for_alpha = anim_value_normalized * 0.6 + 0.2;

        stroke.color = Color::srgba(1., 1., 1., normalized_for_alpha);
    }
}

#[derive(Component)]
pub struct Glow {}

pub fn areas(mut commands: Commands) {
    let rounded_rect = shapes::Rectangle {
        extents: Vec2::new(100.0, 100.0),
        origin: RectangleOrigin::CustomCenter(Vec2::new(100.0, 100.0)),
        radii: Some(BorderRadii::single(10.0)),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rounded_rect),
            ..default()
        },
        Stroke::new(Color::WHITE, 2.0),
        Glow {},
    ));
}
