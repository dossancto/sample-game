use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use interaction_gateway::{
    configurations::player::{InteractionSourceCamera, InterationConfiguration},
    extensions::add_interactable::{Interactable, Interacting, ReadyForInteract},
};
use level_gateway::Screen;
use player_gateway::Player;
use third_parties::packages::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, can_interact.run_if(in_state(Screen::Gameplay)));
}

fn can_interact(
    interactables_q: Query<(&Interactable, )>,
    mut can_interact_q: Query<Entity, With<ReadyForInteract>>,
    interation_configuration: Single<&InterationConfiguration>,
    player: Single<(&GlobalTransform, Entity), With<InteractionSourceCamera>>,
    player_collider: Single<Entity, With<Player>>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
) {
    let (player, _) = player.into_inner();

    let camera_transform = player.compute_transform();

    let interation_configuration = interation_configuration.into_inner();

    let forward = camera_transform.forward();

    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        forward,
        interation_configuration.interaction_distance,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Interactable)
            .with_excluded_entities([*player_collider]),
    );

    for entity in can_interact_q.iter_mut() {
        if let Some(hit) = hit {
            if hit.entity == entity {
                continue;
            }
        }
        commands.entity(entity).remove::<ReadyForInteract>();
    }

    let Some(hit) = hit else {
        return;
    };

    info!("{}", hit.entity);

    let Ok((interactable_item,)) = interactables_q.get(hit.entity) else {
        return;
    };

    info!("Hitting some prop");

    if interactable_item.blocked {
        return;
    }

    commands.entity(hit.entity).insert_if_new(ReadyForInteract);
}
