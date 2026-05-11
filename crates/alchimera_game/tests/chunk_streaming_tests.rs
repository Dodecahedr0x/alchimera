use alchimera_game::{
    build_app,
    player::Player,
    streaming::{ActiveChunk, ChunkStreaming},
};
use alchimera_generation::chunk::ChunkCoord;
use bevy::prelude::{Transform, Vec3, With};

#[test]
fn player_at_origin_requests_origin_and_neighbor_chunks() {
    let mut app = build_app();
    app.update();

    let streaming = app.world().resource::<ChunkStreaming>();

    assert!(streaming.is_requested(ChunkCoord::new(0, 0)));
    assert!(streaming.is_requested(ChunkCoord::new(1, 0)));
    assert!(streaming.is_requested(ChunkCoord::new(-1, 0)));
    assert!(streaming.is_requested(ChunkCoord::new(0, 1)));
    assert!(streaming.is_requested(ChunkCoord::new(0, -1)));
    assert_eq!(streaming.requested_chunks().len(), 9);
}

#[test]
fn moving_player_requests_new_chunk() {
    let mut app = build_app();
    app.update();

    let mut player_query = app
        .world_mut()
        .query_filtered::<&mut Transform, With<Player>>();
    player_query.single_mut(app.world_mut()).translation = Vec3::new(70.0, 0.0, 0.0);
    app.update();

    let streaming = app.world().resource::<ChunkStreaming>();
    assert!(streaming.is_requested(ChunkCoord::new(1, 0)));
    assert!(streaming.is_requested(ChunkCoord::new(2, 0)));
}

#[test]
fn chunks_outside_radius_are_marked_for_despawn() {
    let mut app = build_app();
    app.world_mut().spawn((ActiveChunk {
        coord: ChunkCoord::new(4, 0),
    },));
    app.update();

    let streaming = app.world().resource::<ChunkStreaming>();
    assert!(streaming.is_marked_for_despawn(ChunkCoord::new(4, 0)));
}
