//! Text visualizer for every procedural Alchimera game object archetype.

use alchimera_generation::objects::{object_catalog, ObjectRenderable};

/// Render all known procedural object archetypes using the visual rendering owned by object definitions.
#[must_use]
pub fn render_all_objects() -> String {
    let mut output = String::from("Procedural Alchimera Objects\n============================\n");

    for prototype in object_catalog() {
        output.push('\n');
        output.push_str(&prototype.render_visual().card);
    }

    output
}

/// Extract prototype keys from [`render_all_objects`] output.
#[must_use]
pub fn rendered_prototype_keys(rendered: &str) -> Vec<String> {
    rendered
        .lines()
        .filter_map(|line| line.strip_prefix("key: "))
        .map(str::to_owned)
        .collect()
}
