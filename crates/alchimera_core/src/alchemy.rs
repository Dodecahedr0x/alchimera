//! Pure alchemy trait discovery and two-ingredient experiment rules.

use crate::{
    ids::MaterialId,
    inventory::ItemStack,
    material::{AlchemyTrait, MaterialDefinition},
};

/// Player-facing knowledge discovered by inspecting materials.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AlchemyKnowledge {
    discoveries: Vec<TraitDiscovery>,
}

impl AlchemyKnowledge {
    /// Records all traits currently exposed by a material and returns the discovery.
    pub fn inspect_material(&mut self, material: &MaterialDefinition) -> TraitDiscovery {
        let discovery = TraitDiscovery::new(material.id().clone(), material.traits().to_vec());

        if let Some(existing) = self
            .discoveries
            .iter_mut()
            .find(|existing| existing.material_id == *material.id())
        {
            *existing = discovery.clone();
        } else {
            self.discoveries.push(discovery.clone());
        }

        discovery
    }

    /// Returns true when a trait has been discovered for the material.
    #[must_use]
    pub fn has_discovered_trait(
        &self,
        material_id: &MaterialId,
        alchemy_trait: AlchemyTrait,
    ) -> bool {
        self.discoveries
            .iter()
            .find(|discovery| &discovery.material_id == material_id)
            .is_some_and(|discovery| discovery.discovered_traits.contains(&alchemy_trait))
    }

    #[must_use]
    pub fn discoveries(&self) -> &[TraitDiscovery] {
        &self.discoveries
    }
}

/// Traits found on a single inspected material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitDiscovery {
    material_id: MaterialId,
    discovered_traits: Vec<AlchemyTrait>,
}

impl TraitDiscovery {
    #[must_use]
    pub const fn new(material_id: MaterialId, discovered_traits: Vec<AlchemyTrait>) -> Self {
        Self {
            material_id,
            discovered_traits,
        }
    }

    #[must_use]
    pub const fn material_id(&self) -> &MaterialId {
        &self.material_id
    }

    #[must_use]
    pub fn discovered_traits(&self) -> &[AlchemyTrait] {
        &self.discovered_traits
    }
}

/// Pure two-ingredient experiment rule table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlchemyExperiment {
    rules: Vec<AlchemyExperimentRule>,
}

impl AlchemyExperiment {
    #[must_use]
    pub const fn new(rules: Vec<AlchemyExperimentRule>) -> Self {
        Self { rules }
    }

    /// Combines two materials by matching any trait pair against the rule table.
    #[must_use]
    pub fn combine(
        &self,
        first: &MaterialDefinition,
        second: &MaterialDefinition,
    ) -> AlchemyExperimentResult {
        for first_trait in first.traits() {
            for second_trait in second.traits() {
                if let Some(rule) = self.rule_for(*first_trait, *second_trait) {
                    return AlchemyExperimentResult::success(
                        rule.output.clone(),
                        (*first_trait, *second_trait),
                    );
                }
            }
        }

        AlchemyExperimentResult::failed()
    }

    fn rule_for(
        &self,
        first_trait: AlchemyTrait,
        second_trait: AlchemyTrait,
    ) -> Option<&AlchemyExperimentRule> {
        self.rules
            .iter()
            .find(|rule| rule.matches(first_trait, second_trait))
    }
}

/// A compatible trait pair and the reagent it produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlchemyExperimentRule {
    first_trait: AlchemyTrait,
    second_trait: AlchemyTrait,
    output: ItemStack,
}

impl AlchemyExperimentRule {
    #[must_use]
    pub const fn new(
        first_trait: AlchemyTrait,
        second_trait: AlchemyTrait,
        output: ItemStack,
    ) -> Self {
        Self {
            first_trait,
            second_trait,
            output,
        }
    }

    #[must_use]
    pub const fn first_trait(&self) -> AlchemyTrait {
        self.first_trait
    }

    #[must_use]
    pub const fn second_trait(&self) -> AlchemyTrait {
        self.second_trait
    }

    #[must_use]
    pub const fn output(&self) -> &ItemStack {
        &self.output
    }

    fn matches(&self, first_trait: AlchemyTrait, second_trait: AlchemyTrait) -> bool {
        (self.first_trait == first_trait && self.second_trait == second_trait)
            || (self.first_trait == second_trait && self.second_trait == first_trait)
    }
}

/// Outcome state for an alchemy experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlchemyExperimentStatus {
    Success,
    Failed,
}

/// Result of combining two ingredients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlchemyExperimentResult {
    status: AlchemyExperimentStatus,
    output: Option<ItemStack>,
    matched_traits: Option<(AlchemyTrait, AlchemyTrait)>,
}

impl AlchemyExperimentResult {
    #[must_use]
    pub const fn success(output: ItemStack, matched_traits: (AlchemyTrait, AlchemyTrait)) -> Self {
        Self {
            status: AlchemyExperimentStatus::Success,
            output: Some(output),
            matched_traits: Some(matched_traits),
        }
    }

    #[must_use]
    pub const fn failed() -> Self {
        Self {
            status: AlchemyExperimentStatus::Failed,
            output: None,
            matched_traits: None,
        }
    }

    #[must_use]
    pub const fn status(&self) -> AlchemyExperimentStatus {
        self.status
    }

    #[must_use]
    pub const fn output(&self) -> Option<&ItemStack> {
        self.output.as_ref()
    }

    #[must_use]
    pub const fn matched_traits(&self) -> Option<(AlchemyTrait, AlchemyTrait)> {
        self.matched_traits
    }
}
