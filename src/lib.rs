use std::fmt;

use once_cell::sync::Lazy;
use rand::{seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod namegen;

#[derive(Debug, PartialEq)]
pub enum BuilderError {
    // IncompleteBuilder,
    NameGenFailed,
    UnknownError,
}

#[derive(EnumIter, Debug, PartialEq)]
pub enum ReagentKind {
    Plant,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, EnumIter)]
pub enum ReagentProperty {
    Explosive,
    Volatile,
    // Viscous,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum ReagentEffect {
    Healing,
    Strength,
    Speed,
    Clairvoyance,
    StoneSkin,
    Flight,
    Invisibility,
    Toxic,
    Acidic,
    Freezing,
    Burning,
    Confusion,
    Paralysis,
    Blinding,
    Flashing,
    Hallucination,
}

impl fmt::Display for ReagentKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ReagentEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl fmt::Display for ReagentProperty {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

static INCOMPATIBLES: Lazy<Vec<[ReagentEffect; 2]>> =
    Lazy::new(|| vec![[ReagentEffect::Healing, ReagentEffect::Toxic]]);

#[derive(Debug, PartialEq)]
pub struct Reagent {
    name: String,
    kind: ReagentKind,
    effects: Vec<ReagentEffect>,
    property: Vec<ReagentProperty>,
}

// TODO: Build easy presets? Reagent::plant(), etc

#[derive(Debug, PartialEq)]
pub struct ReagentBuilder {
    kind: Option<ReagentKind>,
    effects: Option<Vec<ReagentEffect>>,
    property: Option<Vec<ReagentProperty>>,
}

impl Default for ReagentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ReagentBuilder {
    pub fn new() -> ReagentBuilder {
        ReagentBuilder {
            kind: None,
            effects: None,
            property: None,
        }
    }

    pub fn with_kind(mut self, kind: ReagentKind) -> ReagentBuilder {
        self.kind = Some(kind);
        self
    }

    pub fn with_random_kind(mut self) -> ReagentBuilder {
        let mut rng = rand::thread_rng();
        self.kind = Some(ReagentKind::iter().choose(&mut rng).unwrap());
        self
    }

    fn add_property(&mut self, prop: ReagentProperty) {
        if self.property.is_none() {
            self.property = Some(vec![prop]);
        } else if let Some(v) = self.property.as_mut() {
            v.push(prop);
            v.sort();
            v.dedup();
        };
    }

    pub fn with_property(mut self, prop: ReagentProperty) -> ReagentBuilder {
        self.add_property(prop);
        self
    }

    pub fn with_random_properties(mut self, num: usize) -> ReagentBuilder {
        let mut rng = rand::thread_rng();

        for _ in 0..num {
            let roll: f32 = rng.gen();
            if roll <= 0.50 {
                return self;
            }
            self.add_property(ReagentProperty::iter().choose(&mut rng).unwrap());
        }

        self
    }

    fn add_effect(&mut self, eff: ReagentEffect) {
        if self.effects.is_none() {
            self.effects = Some(vec![eff]);
        } else if let Some(v) = self.effects.as_mut() {
            v.push(eff);
            v.sort();
            v.dedup();
        }
    }

    pub fn with_effect(mut self, eff: ReagentEffect) -> ReagentBuilder {
        self.add_effect(eff);
        self
    }

    pub fn with_random_effects(mut self, num: usize) -> ReagentBuilder {
        for eff in ReagentEffect::iter().choose_multiple(&mut rand::thread_rng(), num) {
            self.add_effect(eff);
        }
        self
    }

    fn generate_name(&self) -> Result<String, namegen::NameGenError> {
        // Fill in a template using a primary effect + the kind
        // ex: "frost" (Freezing) + "fern" (Plant) = "Frostfern"
        namegen::generate_name(self)
    }

    pub fn build(mut self) -> Result<Reagent, BuilderError> {
        // generate random kind and effect, if missing
        if self.kind.is_none() {
            self = self.with_random_kind();
        }

        if self.effects.is_none() {
            self = self.with_random_effects(1);
        }

        //TODO: figure out easy build (automatically generate effects or kinds)
        //      none of the fields should be required

        if let Ok(name) = self.generate_name() {
            //if the requried fields are in, return the Reagent
            let reagent = Reagent {
                name,
                kind: self.kind.unwrap(),
                effects: self.effects.unwrap(),
                property: self.property.unwrap_or_else(Vec::new),
            };
            Ok(reagent)
        } else {
            Err(BuilderError::NameGenFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_from_new_builder() {
        assert!(ReagentBuilder::new().build().is_ok());
    }

    // #[test]
    // fn cant_build_without_effects() {
    //     let inc = ReagentBuilder::new().with_property(ReagentProperty::Explosive);

    //     assert_eq!(inc.build(), Err(BuilderError::IncompleteBuilder));
    // }

    #[test]
    fn can_build_without_properties() {
        let inc = ReagentBuilder::new()
            .with_effect(ReagentEffect::Healing)
            .with_kind(ReagentKind::Plant);

        assert!(inc.build().is_ok());
    }

    #[test]
    fn adding_property() {
        let builder = ReagentBuilder::new().with_property(ReagentProperty::Explosive);

        assert_eq!(builder.property, Some(vec![ReagentProperty::Explosive]))
    }
    #[test]
    fn adding_property_appends() {
        let builder = ReagentBuilder::new()
            .with_property(ReagentProperty::Explosive)
            .with_property(ReagentProperty::Volatile);

        assert_eq!(
            builder.property,
            Some(vec![ReagentProperty::Explosive, ReagentProperty::Volatile])
        )
    }

    #[test]
    fn adding_dup_property_doesnt_append() {
        let builder = ReagentBuilder::new()
            .with_property(ReagentProperty::Explosive)
            .with_property(ReagentProperty::Explosive);

        assert_eq!(builder.property, Some(vec![ReagentProperty::Explosive]))
    }

    #[test]
    fn adding_effect() {
        let builder = ReagentBuilder::new().with_effect(ReagentEffect::Healing);

        assert_eq!(builder.effects, Some(vec![ReagentEffect::Healing]))
    }

    #[test]
    fn adding_effect_appends() {
        let builder = ReagentBuilder::new()
            .with_effect(ReagentEffect::Healing)
            .with_effect(ReagentEffect::Speed);

        assert_eq!(
            builder.effects,
            Some(vec![ReagentEffect::Healing, ReagentEffect::Speed])
        )
    }

    #[test]
    fn adding_dup_effect_doesnt_append() {
        let builder = ReagentBuilder::new()
            .with_effect(ReagentEffect::Healing)
            .with_effect(ReagentEffect::Healing);

        assert_eq!(builder.effects, Some(vec![ReagentEffect::Healing]))
    }

    #[test]
    fn complete_build() {
        let builder = ReagentBuilder::new()
            .with_kind(ReagentKind::Plant)
            .with_effect(ReagentEffect::Healing)
            .with_property(ReagentProperty::Explosive);
        let end = builder.build();
        assert!(end.is_ok());
    }
}
