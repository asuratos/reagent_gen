use once_cell::sync::Lazy;

mod namegen;

#[derive(Debug, PartialEq)]
enum BuilderError {
    IncompleteBuilder,
    UnknownError,
}

#[derive(Debug, PartialEq)]
enum ReagentKind {
    Plant,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum ReagentProperty {
    Explosive,
    Volatile,
    Viscous,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ReagentEffect {
    Healing,
    Strength,
    Speed,
    Clairvoyance,
    StoneSkin,
    Flight,
    Invisibility,
    Explosive,
    Toxic,
    Freezing,
    Combusting,
    Confusion,
    Paralysis,
    Blinding,
    Flashing,
    Viscous,
    Volatile,
    Hallucination,
}

static INCOMPATIBLES: Lazy<Vec<[ReagentEffect; 2]>> =
    Lazy::new(|| vec![[ReagentEffect::Healing, ReagentEffect::Toxic]]);

#[derive(Debug, PartialEq)]
struct Reagent {
    kind: ReagentKind,
    name: String,
    effects: Vec<ReagentEffect>,
    property: Vec<ReagentProperty>,
}

#[derive(Debug, PartialEq)]
struct ReagentBuilder {
    kind: Option<ReagentKind>,
    effects: Option<Vec<ReagentEffect>>,
    property: Option<Vec<ReagentProperty>>,
}

impl ReagentBuilder {
    pub fn new() -> ReagentBuilder {
        ReagentBuilder {
            kind: None,
            effects: None,
            property: None,
        }
    }

    fn is_incomplete(&self) -> Result<(), BuilderError> {
        //giving myself space to specify which fields are missing in future errors
        if self.kind.is_none() || self.effects.is_none() {
            return Err(BuilderError::IncompleteBuilder);
        }

        Ok(())
    }

    pub fn with_kind(mut self, kind: ReagentKind) -> ReagentBuilder {
        self.kind = Some(kind);
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

    fn generate_name(&self) -> String {
        // TODO: name should be generated here

        // Fill in a template using a primary effect + the kind
        // ex: "frost" (Freezing) + "fern" (Plant) = "Frostfern"

        "test".to_string()
    }

    pub fn build(self) -> Result<Reagent, BuilderError> {
        //check if required fields are None
        self.is_incomplete()?;

        //if the requried fields are in, return the Reagent
        let reagent = Reagent {
            name: self.generate_name(),
            kind: self.kind.unwrap(),
            effects: self.effects.unwrap(),
            property: self.property.unwrap_or_else(Vec::new),
        };

        Ok(reagent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cant_build_from_new_builder() {
        let inc = ReagentBuilder::new();

        assert_eq!(inc.build(), Err(BuilderError::IncompleteBuilder));
    }

    #[test]
    fn cant_build_without_effects() {
        let inc = ReagentBuilder::new().with_property(ReagentProperty::Explosive);

        assert_eq!(inc.build(), Err(BuilderError::IncompleteBuilder));
    }

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
            .with_property(ReagentProperty::Viscous);

        assert_eq!(
            builder.property,
            Some(vec![ReagentProperty::Explosive, ReagentProperty::Viscous])
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
            .with_effect(ReagentEffect::Healing)
            .with_property(ReagentProperty::Explosive);
    }
}
