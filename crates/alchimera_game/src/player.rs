//! Player runtime skeletons.

use bevy::prelude::{
    App, Camera3d, Commands, Component, Entity, Event, EventReader, Plugin, Startup, Transform,
    Update, Vec3,
};

/// Marker component for the controllable player entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Default)]
pub struct Player;

/// Tunable player movement statistics.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct PlayerStats {
    /// Horizontal walking speed in meters per second.
    pub walk_speed: f32,
    /// Prototype jump impulse/strength used by later physics integration.
    pub jump_strength: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            walk_speed: 5.0,
            jump_strength: 6.0,
        }
    }
}

/// Latest movement intent calculated from input for the player controller.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct MovementIntent {
    /// Normalized horizontal movement direction in world/local space.
    pub direction: Vec3,
    /// Whether the player requested a jump this frame.
    pub wants_jump: bool,
}

impl Default for MovementIntent {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            wants_jump: false,
        }
    }
}

/// Companion camera marker tied to a player entity.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct PlayerCamera {
    pub player: Entity,
}

impl PlayerCamera {
    /// Default over-the-shoulder camera offset from the player.
    pub const DEFAULT_OFFSET: Vec3 = Vec3::new(0.0, 2.0, 6.0);
}

/// Input event consumed by the player skeleton to update movement intent.
#[derive(Debug, Clone, Copy, PartialEq, Event)]
pub struct PlayerMovementInput {
    /// Desired movement direction. Non-zero values are normalized by the system.
    pub direction: Vec3,
    /// Whether jump is currently requested.
    pub wants_jump: bool,
}

/// Registers player spawning and movement-intent systems.
#[derive(Debug, Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovementInput>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, apply_player_movement_input);
    }
}

fn spawn_player(mut commands: Commands) {
    let player = commands
        .spawn((
            Player,
            PlayerStats::default(),
            MovementIntent::default(),
            Transform::default(),
        ))
        .id();
    commands.spawn((
        PlayerCamera { player },
        Camera3d::default(),
        Transform::from_translation(PlayerCamera::DEFAULT_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn apply_player_movement_input(
    mut events: EventReader<PlayerMovementInput>,
    mut players: bevy::prelude::Query<&mut MovementIntent, bevy::prelude::With<Player>>,
) {
    let Some(input) = events.read().last().copied() else {
        return;
    };
    let direction = input.direction.normalize_or_zero();
    for mut intent in &mut players {
        intent.direction = direction;
        intent.wants_jump = input.wants_jump;
    }
}
