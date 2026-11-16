use bag_gateway::components::bag::Bag;
use bag_gateway::components::collectable_item::CollectableItemStats;
use bevy::prelude::*;
use interaction_gateway::{
    configurations::player::InteractionSourceCamera, extensions::add_interactable::ReadyForInteract,
};

const ITEM_STATUS_HEIGHT: f32 = 0.25;
const ITEM_STATUS_WIDTH: f32 = 260.0;
const ITEM_STATUS_HEIGHT_IN_UI: f32 = 140.0;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_item_status);
    app.add_observer(clear_item_status);

    app.add_observer(show_item_status);
    app.add_observer(hide_item_status);
    app.add_systems(Update, update_item_status);
}

#[derive(Component)]
struct ItemStatusText {
    item: Option<Entity>,
}

#[derive(Component)]
struct ItemStatusVolumeText;

#[derive(Component)]
struct ItemStatusMassText;

#[derive(Component)]
struct ItemStatusCarryText;

fn setup_item_status(_on: On<Add, InteractionSourceCamera>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("Collectable item tooltip"),
            ItemStatusText { item: None },
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                width: px(ITEM_STATUS_WIDTH),
                padding: UiRect::all(px(12.0)),
                border: UiRect::all(px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: px(5.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.95)),
            BorderColor::all(Color::srgb(1.0, 0.65, 0.1)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Collectable item"),
                TextFont::from_font_size(20.0),
                TextColor(Color::srgb(1.0, 0.65, 0.1)),
            ));

            parent.spawn((
                Node {
                    width: percent(100.0),
                    height: px(2.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 0.65, 0.1)),
            ));

            parent.spawn((
                Text::new(""),
                ItemStatusVolumeText,
                TextFont::from_font_size(16.0),
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new(""),
                ItemStatusMassText,
                TextFont::from_font_size(16.0),
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new(""),
                ItemStatusCarryText,
                TextFont::from_font_size(18.0),
                TextColor(Color::WHITE),
            ));
        });
}

fn clear_item_status(
    _on: On<Remove, InteractionSourceCamera>,
    item_status_q: Query<Entity, With<ItemStatusText>>,
    mut commands: Commands,
) {
    for entity in item_status_q.iter() {
        commands.entity(entity).despawn();
    }
}

fn show_item_status(
    on: On<Add, ReadyForInteract>,
    mut item_status_q: Query<(&mut ItemStatusText, &mut Visibility)>,
) {
    for (mut item_status, mut visibility) in item_status_q.iter_mut() {
        item_status.item = Some(on.entity);
        *visibility = Visibility::Visible;
    }
}

fn hide_item_status(
    on: On<Remove, ReadyForInteract>,
    mut item_status_q: Query<(&mut ItemStatusText, &mut Visibility)>,
) {
    for (mut item_status, mut visibility) in item_status_q.iter_mut() {
        if item_status.item == Some(on.entity) {
            item_status.item = None;
            *visibility = Visibility::Hidden;
        }
    }
}

fn update_item_status(
    mut item_status_q: Query<(&mut ItemStatusText, &mut Node, &mut Visibility)>,
    item_q: Query<(&GlobalTransform, &CollectableItemStats)>,
    bag_q: Query<&Bag>,
    camera_q: Single<(&Camera, &GlobalTransform), With<InteractionSourceCamera>>,
    mut status_text_q: ParamSet<(
        Query<&mut Text, With<ItemStatusVolumeText>>,
        Query<&mut Text, With<ItemStatusMassText>>,
        Query<(&mut Text, &mut TextColor), With<ItemStatusCarryText>>,
    )>,
) {
    let (camera, camera_transform) = camera_q.into_inner();

    for (mut status, mut node, mut visibility) in item_status_q.iter_mut() {
        let Some(item) = status.item else {
            *visibility = Visibility::Hidden;
            continue;
        };

        let Ok((item_transform, stats)) = item_q.get(item) else {
            status.item = None;
            *visibility = Visibility::Hidden;
            node.display = Display::None;
            continue;
        };

        let can_carry = bag_q
            .iter()
            .next()
            .is_some_and(|bag| bag.can_add_volume(stats.volume) && bag.can_add_mass(stats.mass()));

        let (carry_status, carry_color) = if can_carry {
            ("Can carry", Color::srgb(0.1, 0.9, 0.1))
        } else {
            ("Cannot carry", Color::srgb(1.0, 0.2, 0.2))
        };

        if let Ok(mut text) = status_text_q.p0().single_mut() {
            text.0 = format!("Volume: {:.2}", stats.volume);
        }

        if let Ok(mut text) = status_text_q.p1().single_mut() {
            text.0 = format!("Mass: {:.2}", stats.mass());
        }

        if let Ok((mut text, mut color)) = status_text_q.p2().single_mut() {
            text.0 = carry_status.to_string();
            color.0 = carry_color;
        }

        let item_position = item_transform.translation() + Vec3::Y * ITEM_STATUS_HEIGHT;

        let Ok(viewport_position) = camera.world_to_viewport(camera_transform, item_position)
        else {
            *visibility = Visibility::Hidden;
            node.display = Display::None;
            continue;
        };

        *visibility = Visibility::Visible;
        node.display = Display::Flex;
        node.left = px(viewport_position.x - ITEM_STATUS_WIDTH / 2.0);
        node.top = px(viewport_position.y - ITEM_STATUS_HEIGHT_IN_UI);
    }
}
