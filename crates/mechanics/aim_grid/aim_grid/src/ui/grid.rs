use aim_grid_gateway::components::hand_status::{LeftHandCurrentCell, RightHandCurrentCell};
use bevy::{prelude::*, window::PrimaryWindow};
use level_gateway::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_aim_point);
    app.add_systems(Update, update_point_place);
}

#[derive(Component)]
struct SelectedLeftHandGridIndicatorText;

#[derive(Component)]
struct SelectedRightHandGridIndicatorText;

fn setup_aim_point(mut commands: Commands) {
    commands.spawn((
        Name::new("Selected Left hand grid point"),
        Text::new("|"),
        SelectedLeftHandGridIndicatorText,
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Selected Right hand grid point"),
        Text::new("-"),
        SelectedRightHandGridIndicatorText,
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn update_point_place(
    left_hand_current_cell: Single<&LeftHandCurrentCell>,
    right_hand_current_cell: Single<&RightHandCurrentCell>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut left_hand_node: Single<
        &mut Node,
        (
            With<SelectedLeftHandGridIndicatorText>,
            Without<SelectedRightHandGridIndicatorText>,
        ),
    >,
    mut right_hand_node: Single<
        &mut Node,
        (
            With<SelectedRightHandGridIndicatorText>,
            Without<SelectedLeftHandGridIndicatorText>,
        ),
    >,
) {
    // TODO: Move that to a Configuration Resource.
    let max_x_grid = 4f32;
    let max_y_grid = 3f32;

    let width = window.width();
    let height = window.height();

    let cell_width = width / max_x_grid;
    let cell_height = height / max_y_grid;

    let left_hand_x_offset = cell_width * left_hand_current_cell.current_cell.x as f32;
    let left_hand_y_offset = cell_height * left_hand_current_cell.current_cell.y as f32;

    let left_hand_x_pixels = left_hand_x_offset + (cell_width / 2.);
    let left_hand_y_pixels = left_hand_y_offset + (cell_height / 2.);

    left_hand_node.top = px(left_hand_y_pixels);
    left_hand_node.left = px(left_hand_x_pixels);

    let right_hand_x_offset = cell_width * right_hand_current_cell.current_cell.x as f32;
    let right_hand_y_offset = cell_height * right_hand_current_cell.current_cell.y as f32;

    let right_hand_x_pixels = right_hand_x_offset + (cell_width / 2.);
    let right_hand_y_pixels = right_hand_y_offset + (cell_height / 2.);

    right_hand_node.top = px(right_hand_y_pixels);
    right_hand_node.left = px(right_hand_x_pixels);
}
