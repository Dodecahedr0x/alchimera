use alchimera_core::seed::WorldSeed;
use alchimera_game::objects::{spawn_generated_object, WorldObject};
use alchimera_generation::{
    chunk::ChunkCoord,
    objects::{GeneratedObject, ObjectGenerator, ProceduralGameObject},
};
use bevy::prelude::{Name, Transform, World};

#[test]
fn spawn_generated_object_adds_world_object_component() {
    let mut world = World::new();
    let generated = sample_object();

    let entity = spawn_generated_object(&mut world, &generated);

    let world_object = world.entity(entity).get::<WorldObject>().unwrap();
    assert_eq!(world_object.id, generated.id());
    assert_eq!(world_object.prototype_key, generated.prototype_key());
}

#[test]
fn spawned_object_preserves_stable_object_id() {
    let mut world = World::new();
    let generated = sample_object();

    let entity = spawn_generated_object(&mut world, &generated);

    assert_eq!(
        world.entity(entity).get::<WorldObject>().unwrap().id,
        generated.id()
    );
    assert!(world.entity(entity).get::<Name>().is_some());
}

#[test]
fn spawned_object_has_transform_from_generated_instance() {
    let mut world = World::new();
    let generated = sample_object();
    let generated_transform = generated.transform();

    let entity = spawn_generated_object(&mut world, &generated);

    let transform = world.entity(entity).get::<Transform>().unwrap();
    assert_eq!(
        transform.translation.to_array(),
        generated_transform.translation
    );
    assert_eq!(transform.scale.to_array(), [generated_transform.scale; 3]);
}

fn sample_object() -> GeneratedObject {
    ObjectGenerator::default()
        .generate_chunk(WorldSeed::new(0x0B1EC7), ChunkCoord::new(4, -2))
        .into_iter()
        .next()
        .expect("sample seed should generate at least one object")
}
