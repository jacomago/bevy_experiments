use bevy::{ecs::system::Resource, prelude::*};

/// Remove any entities with the components
pub fn cleanup_components<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Cleanup any resources of a type
pub fn cleanup_resource<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}
