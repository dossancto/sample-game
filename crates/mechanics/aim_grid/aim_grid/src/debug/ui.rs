use aim_grid_gateway::components::hand_status::{LeftHandCurrentCell, RightHandCurrentCell};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_debug_ui);
    app.add_systems(Update, render_debug_ui);
}

#[derive(Component)]
struct GridDebugCurrentStateDebugUI;

fn setup_debug_ui(mut commands: Commands) {
    commands.spawn((        Name::new("Bag Content volume Text"),
        Text::new("Cell hand debug UI"),
        GridDebugCurrentStateDebugUI,
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
));
}

fn render_debug_ui(
    left_hand_current_cell: Single<&LeftHandCurrentCell>,
    right_hand_current_cell: Single<&RightHandCurrentCell>,
    mut text: Single<&mut Text, With<GridDebugCurrentStateDebugUI>>,
) {
    let grid_txt = format!(
        "Left hand cell: {:?}/{:?}\nRight hand cell: {:?}/{:?}",
        left_hand_current_cell.current_cell.x, left_hand_current_cell.current_cell.y,
        right_hand_current_cell.current_cell.x, right_hand_current_cell.current_cell.y
    );

    text.0 = grid_txt;
}
