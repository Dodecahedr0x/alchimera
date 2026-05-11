//! Input action definitions for the runtime shell.

/// High-level player/gameplay actions used by the input layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Interact,
    HotbarSlot(u8),
}

/// Prototype binding names for high-level actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputBinding {
    pub action: InputAction,
    pub key: &'static str,
}

/// Static input map used until the final input backend is wired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputMap {
    bindings: Vec<InputBinding>,
}

impl InputMap {
    #[must_use]
    pub fn new(bindings: Vec<InputBinding>) -> Self {
        Self { bindings }
    }

    #[must_use]
    pub fn bindings(&self) -> &[InputBinding] {
        &self.bindings
    }

    #[must_use]
    pub fn contains_action(&self, action: InputAction) -> bool {
        self.bindings.iter().any(|binding| binding.action == action)
    }
}

#[must_use]
pub fn default_input_map() -> InputMap {
    InputMap::new(vec![
        InputBinding {
            action: InputAction::MoveForward,
            key: "W",
        },
        InputBinding {
            action: InputAction::MoveBackward,
            key: "S",
        },
        InputBinding {
            action: InputAction::MoveLeft,
            key: "A",
        },
        InputBinding {
            action: InputAction::MoveRight,
            key: "D",
        },
        InputBinding {
            action: InputAction::Jump,
            key: "Space",
        },
        InputBinding {
            action: InputAction::Interact,
            key: "E",
        },
        InputBinding {
            action: InputAction::HotbarSlot(0),
            key: "1",
        },
        InputBinding {
            action: InputAction::HotbarSlot(1),
            key: "2",
        },
        InputBinding {
            action: InputAction::HotbarSlot(2),
            key: "3",
        },
        InputBinding {
            action: InputAction::HotbarSlot(3),
            key: "4",
        },
        InputBinding {
            action: InputAction::HotbarSlot(4),
            key: "5",
        },
        InputBinding {
            action: InputAction::HotbarSlot(5),
            key: "6",
        },
        InputBinding {
            action: InputAction::HotbarSlot(6),
            key: "7",
        },
        InputBinding {
            action: InputAction::HotbarSlot(7),
            key: "8",
        },
        InputBinding {
            action: InputAction::HotbarSlot(8),
            key: "9",
        },
    ])
}
