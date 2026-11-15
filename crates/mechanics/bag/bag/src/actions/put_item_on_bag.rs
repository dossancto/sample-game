use bag_gateway::components::collectable_item::CollectableItemStats;
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
) {
    let Ok(stats) = stats_q.get(on.entity) else {
        return;
    };

    commands.entity(on.entity).despawn();
}
