use alchimera_generation::objects::{object_catalog, ObjectRenderable};

#[test]
fn object_rendering_logic_produces_visual_cards_for_every_prototype() {
    for prototype in object_catalog() {
        let rendered = prototype.render_visual();

        assert_eq!(rendered.key, prototype.key.as_str());
        assert!(
            rendered.card.starts_with("┌"),
            "visual cards use box rendering"
        );
        assert!(rendered.card.contains(prototype.display_name));
        assert!(rendered.card.contains(prototype.ascii_icon));
        assert!(rendered.card.ends_with("┘\n"));
    }
}
