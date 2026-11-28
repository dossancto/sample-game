use bevy::prelude::*;
use level_gateway::Screen;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), spawn_crosshair);

    // app.add_observer(render_can_interact);
    // app.add_observer(remove_can_interact);
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct CrosshairState;

fn spawn_crosshair(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Crosshair"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Crosshair Image"),
                CrosshairState::default(),
                Text::new("."),
            ));
        });
}

// fn render_can_interact(
//     _on: On<Add, ReadyForInteract>,
//     mut crosshair_text: Single<&mut Text, With<CrosshairState>>,
//     ) {
//     crosshair_text.0 = "[]".to_string();
// }
//
// fn remove_can_interact(
//     _on: On<Remove, ReadyForInteract>,
//     mut crosshair_text: Single<&mut Text, With<CrosshairState>>,
//     ) {
//     crosshair_text.0 = ".".to_string();
// }
