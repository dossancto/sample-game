use bag_gateway::components::{bag::Bag, collectable_item::CollectableItemStats};
use bevy::prelude::*;
use interaction_gateway::handlers::collect::InteractionEnd;
use level::props::general::actions::lamp_sitting::LampSitting;

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
        // TODO: notify out of space
        return;
    }

    if bag.can_add_mass(stats.mass()) == false {
        // TODO: notify too heavy
        return;
    }

    bag.add_collectable_item(stats);

    commands.entity(on.entity).despawn();
}
