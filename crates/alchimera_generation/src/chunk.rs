//! Chunk generation and coordinate math.

/// Default width/depth of an invisible generation chunk in world meters.
pub const CHUNK_SIZE_METERS: f32 = 64.0;

/// Integer coordinate for an invisible world-generation chunk on the X/Z plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoord {
    /// Creates a chunk coordinate from integer X/Z chunk indices.
    #[must_use]
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Converts a world-space X/Z position into a chunk coordinate.
    ///
    /// Negative positions intentionally use mathematical floor semantics so
    /// `-0.01` meters maps to chunk `-1`, not chunk `0`.
    #[must_use]
    pub fn from_world_position(x: f32, z: f32) -> Self {
        Self {
            x: world_axis_to_chunk(x),
            z: world_axis_to_chunk(z),
        }
    }

    /// Returns inclusive-exclusive world bounds covered by this chunk.
    #[must_use]
    pub fn world_bounds(self) -> ChunkWorldBounds {
        let min_x = self.x as f32 * CHUNK_SIZE_METERS;
        let min_z = self.z as f32 * CHUNK_SIZE_METERS;
        ChunkWorldBounds {
            min_x,
            min_z,
            max_x: min_x + CHUNK_SIZE_METERS,
            max_z: min_z + CHUNK_SIZE_METERS,
        }
    }
}

/// World-space bounds for an invisible chunk on the X/Z plane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkWorldBounds {
    pub min_x: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_z: f32,
}

fn world_axis_to_chunk(value: f32) -> i32 {
    (value / CHUNK_SIZE_METERS).floor() as i32
}
