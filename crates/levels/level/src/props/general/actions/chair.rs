use bevy::prelude::*;
use interaction_gateway::{extensions::add_interactable::AddInteractable, handlers::collect::InteractionEnd};

use crate::props::general::model::Chair;

pub(super) fn plugin(app: &mut App){
    app.add_interaction::<Chair>();
    app.add_observer(on_collect_chair);
}

fn on_collect_chair(on: On<InteractionEnd<Chair>>, mut commands: Commands) {
    commands.entity(on.entity).despawn();
    info!("Collected");
}

