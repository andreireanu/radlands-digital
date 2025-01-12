use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const ANIMATION_DURATION: f32 = 0.1;

pub fn on_test(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<(&mut Stroke, &mut Glow)>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Space => {
                for (_, mut glow) in query.iter_mut().filter(|(_, glow)| !glow.state) {
                    glow.state = true;
                }
            }
            Key::Enter => {
                for (_, mut glow) in query.iter_mut().filter(|(_, glow)| glow.state) {
                    glow.state = false;
                }
            }
            _ => {}
        }
    }
}

pub fn animate_glow(time: Res<Time>, mut query: Query<(&mut Stroke, &mut Glow)>) {
    for (mut stroke, _) in query.iter_mut().filter(|(_, glow)| glow.state) {
        // Get the elapsed seconds since the app started
        let elapsed = time.elapsed_secs();

        let anim_value = (elapsed / ANIMATION_DURATION).sin(); // Oscillator
        let anim_value_normalized = (anim_value + 1.0) * 0.5; // Normalize to [0, 1]
        let normalized_for_alpha = anim_value_normalized * 0.6 + 0.2;

        stroke.color.set_alpha(normalized_for_alpha);
    }
}

pub fn stop_glow(mut query: Query<(&mut Stroke, &mut Glow)>) {
    for (mut stroke, _) in query
        .iter_mut()
        .filter(|(stroke, glow)| !glow.state && stroke.color.alpha() > 0.0)
    {
        stroke.color.set_alpha(0.);
    }
}

#[derive(Component)]
pub struct Glow {
    state: bool,
}

pub fn areas(mut commands: Commands) {
    let rounded_rect = shapes::Rectangle {
        extents: Vec2::new(148., 206.0),
        origin: RectangleOrigin::CustomCenter(Vec2::new(-1064.0, 742.0)),
        radii: Some(BorderRadii::single(10.0)),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rounded_rect),
            ..default()
        },
        Stroke::new(Color::srgba(255., 255., 255., 0.), 2.0),
        Glow { state: false },
    ));
}
