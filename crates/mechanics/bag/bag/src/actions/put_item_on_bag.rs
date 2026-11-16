use bag_gateway::components::{bag::Bag, collectable_item::CollectableItemStats};
use bevy::prelude::*;
use interaction_gateway::handlers::collect::InteractionEnd;
use level::props::general::actions::lamp_sitting::LampSitting;
use menus_gateway::ToastEvent;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(put_item_on_bag);
}

fn put_item_on_bag(
    on: On<InteractionEnd<LampSitting>>,
    mut commands: Commands,
    stats_q: Query<&CollectableItemStats>,
    mut bag: Single<&mut Bag>,
) {
    let Ok(stats) = stats_q.get(on.entity) else {
        return;
    };

    if bag.can_add_volume(stats.volume) == false {
        commands.trigger(ToastEvent::warning(
            "There is not enough space in your bag.",
            Duration::from_secs(3),
        ));
        return;
    }

    if bag.can_add_mass(stats.mass()) == false {
        commands.trigger(ToastEvent::warning(
            "This item is too heavy for your bag.",
            Duration::from_secs(3),
        ));
        return;
    }

    bag.add_collectable_item(stats);

    commands.entity(on.entity).despawn();
}
