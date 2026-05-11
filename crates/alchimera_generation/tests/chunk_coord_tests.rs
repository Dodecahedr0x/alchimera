use alchimera_generation::chunk::{ChunkCoord, CHUNK_SIZE_METERS};

#[test]
fn chunk_coord_origin_maps_to_origin_chunk() {
    assert_eq!(
        ChunkCoord::from_world_position(0.0, 0.0),
        ChunkCoord::new(0, 0)
    );
}

#[test]
fn chunk_coord_positive_world_position_maps_to_expected_chunk() {
    assert_eq!(
        ChunkCoord::from_world_position(127.99, 64.0),
        ChunkCoord::new(1, 1)
    );
}

#[test]
fn chunk_coord_negative_world_position_floors_to_negative_chunk() {
    assert_eq!(
        ChunkCoord::from_world_position(-0.01, -64.01),
        ChunkCoord::new(-1, -2)
    );
}

#[test]
fn chunk_coord_world_bounds_are_64m_square() {
    let bounds = ChunkCoord::new(-2, 3).world_bounds();

    assert_eq!(CHUNK_SIZE_METERS, 64.0);
    assert_eq!(bounds.min_x, -128.0);
    assert_eq!(bounds.min_z, 192.0);
    assert_eq!(bounds.max_x, -64.0);
    assert_eq!(bounds.max_z, 256.0);
}
