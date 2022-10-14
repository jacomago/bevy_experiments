use bevy::{ecs::system::Resource, prelude::*};

pub fn cleanup_components<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn cleanup_resource<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}
