//! Game object runtime skeletons.

use alchimera_core::ids::ObjectId;
use alchimera_generation::objects::{GeneratedObject, ObjectPrototypeKey, ProceduralGameObject};
use bevy::prelude::{Component, Entity, Name, Quat, Transform, Vec3, World};

/// Runtime marker for an object spawned into the Bevy world.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct WorldObject {
    pub id: ObjectId,
    pub prototype_key: ObjectPrototypeKey,
}

/// Spawns a deterministic generated object into the Bevy ECS world.
pub fn spawn_generated_object(world: &mut World, generated: &GeneratedObject) -> Entity {
    let object_transform = generated.transform();
    world
        .spawn((
            WorldObject {
                id: generated.id(),
                prototype_key: generated.prototype_key(),
            },
            Name::new(format!(
                "{}:{}",
                generated.prototype_key().as_str(),
                generated.id()
            )),
            Transform {
                translation: Vec3::from_array(object_transform.translation),
                rotation: Quat::from_rotation_y(object_transform.yaw_radians),
                scale: Vec3::splat(object_transform.scale),
            },
        ))
        .id()
}
