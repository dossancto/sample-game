use bevy::prelude::*;
use menus_gateway::{ToastEvent, ToastKind};

#[derive(Component)]
struct ToastContainer;

#[derive(Component)]
struct Toast {
    id: String,
    lifetime: Timer,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_container)
        .add_systems(Update, tick_toasts)
        .add_observer(show_toast);
}

fn setup_container(mut commands: Commands) {
    commands.spawn((
        Name::new("Toast container"),
        ToastContainer,
        Node {
            position_type: PositionType::Absolute,
            top: px(28),
            right: px(28),
            width: px(380),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            row_gap: px(10),
            ..default()
        },
    ));
}

fn show_toast(
    on: On<ToastEvent>,
    containers: Query<Entity, With<ToastContainer>>,
    mut toasts: Query<&mut Toast>,
    mut commands: Commands,
) {
    if let Some(mut toast) = toasts.iter_mut().find(|toast| toast.id == on.id) {
        toast.lifetime = Timer::new(on.duration, TimerMode::Once);
        return;
    }

    let Some(container) = containers.iter().next() else {
        return;
    };

    commands.entity(container).with_children(|parent| {
        parent.spawn((
            Name::new("Toast"),
            Toast {
                id: on.id.clone(),
                lifetime: Timer::new(on.duration, TimerMode::Once),
            },
            Node {
                width: percent(100),
                min_height: px(48),
                padding: UiRect::axes(px(16), px(12)),
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(background_color(on.kind)),
            Text::new(on.message.clone()),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

fn background_color(kind: ToastKind) -> Color {
    match kind {
        ToastKind::Info => Color::srgb(0.08, 0.35, 0.75),
        ToastKind::Warning => Color::srgb(0.82, 0.48, 0.04),
        ToastKind::Error => Color::srgb(0.75, 0.12, 0.12),
    }
}

fn tick_toasts(
    time: Res<Time>,
    mut commands: Commands,
    mut toasts: Query<(Entity, &mut Toast)>,
) {
    for (entity, mut toast) in &mut toasts {
        toast.lifetime.tick(time.delta());

        if toast.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
