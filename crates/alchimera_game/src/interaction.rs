//! Interaction targeting resources and events.

use alchimera_core::ids::ObjectId;
use bevy::prelude::{App, Entity, Event, EventReader, Plugin, Query, ResMut, Resource, Update};

use crate::objects::WorldObject;

/// Raycast result emitted by the input/camera layer for interaction targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub enum InteractionRaycast {
    Hit(Entity),
    Miss,
}

/// Current object selected as the interaction target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractionTarget {
    pub entity: Entity,
    pub object_id: ObjectId,
}

/// Resource storing the current interaction target, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub struct CurrentInteractionTarget {
    target: Option<InteractionTarget>,
}

impl CurrentInteractionTarget {
    #[must_use]
    pub const fn target(self) -> Option<InteractionTarget> {
        self.target
    }

    #[must_use]
    pub fn entity(&self) -> Option<Entity> {
        self.target.map(|target| target.entity)
    }

    #[must_use]
    pub fn object_id(&self) -> Option<ObjectId> {
        self.target.map(|target| target.object_id)
    }
}

/// Registers interaction targeting resources and event systems.
#[derive(Debug, Default)]
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentInteractionTarget>()
            .add_event::<InteractionRaycast>()
            .add_systems(Update, update_interaction_target);
    }
}

fn update_interaction_target(
    mut events: EventReader<InteractionRaycast>,
    objects: Query<&WorldObject>,
    mut current: ResMut<CurrentInteractionTarget>,
) {
    for event in events.read() {
        match *event {
            InteractionRaycast::Hit(entity) => {
                current.target = objects.get(entity).ok().map(|object| InteractionTarget {
                    entity,
                    object_id: object.id,
                });
            }
            InteractionRaycast::Miss => {
                current.target = None;
            }
        }
    }
}
