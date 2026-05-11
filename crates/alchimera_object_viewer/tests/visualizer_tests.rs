use alchimera_object_viewer::{render_all_objects, rendered_prototype_keys};

#[test]
fn visualizer_renders_every_registered_procedural_object() {
    let output = render_all_objects();
    let rendered_keys = rendered_prototype_keys(&output);
    let catalog_keys: Vec<_> = alchimera_generation::objects::object_catalog()
        .iter()
        .map(|prototype| prototype.key.as_str().to_owned())
        .collect();

    assert_eq!(rendered_keys, catalog_keys);
    assert!(output.contains("Procedural Alchimera Objects"));
}
