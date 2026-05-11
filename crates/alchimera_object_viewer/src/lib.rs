//! Text visualizer for every procedural Alchimera game object archetype.

use alchimera_generation::objects::object_catalog;

/// Render all known procedural object archetypes in a stable text format.
#[must_use]
pub fn render_all_objects() -> String {
    let mut output = String::from("Procedural Alchimera Objects\n============================\n");

    for prototype in object_catalog() {
        output.push('\n');
        output.push_str("key: ");
        output.push_str(prototype.key.as_str());
        output.push('\n');
        output.push_str("name: ");
        output.push_str(prototype.display_name);
        output.push('\n');
        output.push_str("spawn_weight: ");
        output.push_str(&prototype.spawn_weight.to_string());
        output.push('\n');
        output.push_str(prototype.ascii_icon);
        output.push('\n');
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
