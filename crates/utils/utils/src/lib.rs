use bevy::{
    ecs::query::{QueryData, QueryFilter, ROQueryItem},
    ecs::system::Query,
    prelude::{ChildOf, Entity},
};

/// Finds the closest ancestor (including `entity`) that matches `query`.
///
/// This is useful when an engine-generated child entity is the one returned by
/// a query, but gameplay components are stored on an owning parent entity. The
/// query may use any read-only query data and filter, for example:
///
/// ```ignore
/// let result = find_in_ancestors(hit_entity, &interactables, &parents);
/// ```
///
/// The returned query item is read-only, even when `Q` contains mutable query
/// data. Use the returned entity with a separate mutable query when a mutation
/// is needed.
pub fn find_in_ancestors<'w, 's, 'q, Q, F>(
    entity: Entity,
    query: &'q Query<'w, 's, Q, F>,
    parents: &Query<'_, '_, &ChildOf>,
) -> Option<(Entity, ROQueryItem<'q, 's, Q>)>
where
    Q: QueryData,
    F: QueryFilter,
{
    let mut entity = entity;

    loop {
        if let Ok(item) = query.get(entity) {
            return Some((entity, item));
        }

        let Ok(child_of) = parents.get(entity) else {
            return None;
        };
        entity = child_of.parent();
    }
}
