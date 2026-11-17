use bevy::prelude::*;
use interaction_gateway::handlers::collect::InteractionEnd;
use level::props::general::actions::burning_logs::BurningLogs;
use money_gateway::{CollectableMoney, MoneyBag};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(collect_money);
}

fn collect_money(
    on: On<InteractionEnd<BurningLogs>>,
    mut money_bag: Single<&mut MoneyBag>,
    money_to_collect: Query<&CollectableMoney>,
    mut commands: Commands,
) {
    let Ok(money) = money_to_collect.get(on.entity) else {
        return;
    };

    money_bag.add_money(money.value);

    commands.entity(on.entity).despawn();
}
