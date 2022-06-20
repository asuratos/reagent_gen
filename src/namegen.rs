use std::collections::HashMap;
use std::fs;

use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use ron;

use crate::ReagentBuilder;

// TODO: this should eventually be read from a raw file, like a JSON or RON
static NAMES: Lazy<HashMap<String, Vec<String>>> = Lazy::new(|| {
    ron::from_str(
        fs::read_to_string("src/data/names.ron")
            .expect("Could not open file!")
            .as_str(),
    )
    .expect("Could not deserialize!")
});

pub enum NameGenError {
    UnknownProperty,
    UninitializedKind,
    UninitializedEffect,
    EmptyNameList,
}

pub fn lookup_name_fragment<T: ToString>(prop: T) -> Result<String, NameGenError> {
    let lc_prop = prop.to_string().to_lowercase();

    if !NAMES.contains_key(lc_prop.as_str()) {
        return Err(NameGenError::UnknownProperty);
    }

    let names_list = NAMES.get(lc_prop.as_str()).unwrap();
    let frag = names_list.choose(&mut rand::thread_rng());

    if frag.is_none() {
        return Err(NameGenError::EmptyNameList);
    }

    Ok(frag.unwrap().to_string())
}

pub fn new_name(builder: &ReagentBuilder) -> Result<String, NameGenError> {
    // get &str version of kind for lookup
    // lookup name fragment for kind

    let kind: String;
    if let Some(k) = &builder.kind {
        kind = lookup_name_fragment(k)?;
    } else {
        return Err(NameGenError::UninitializedKind);
    }

    // get &str of one of the effect for lookup
    // lookup name fragment for effect
    let eff: String;
    if let Some(e) = &builder.effects {
        // TODO: change this to randomly pick one?
        eff = lookup_name_fragment(&e[0])?;
    } else {
        return Err(NameGenError::UninitializedEffect);
    }

    // use template
    let template = "{{prop}}{{kind}}";

    Ok(template
        .replace("{{prop}}", &eff)
        .replace("{{kind}}", &kind))
}

#[cfg(test)]
mod tests {
    use crate::{ReagentEffect, ReagentKind};

    use super::*;

    use strum::IntoEnumIterator;

    #[test]
    fn unknown_property() {
        assert!(lookup_name_fragment("tag doesn't exist").is_err());
    }

    #[test]
    fn name_fragment_args_are_case_insensitive() {
        assert!(lookup_name_fragment("Plant").is_ok());
        assert!(lookup_name_fragment("plant").is_ok());
    }

    #[test]
    fn names_dict_is_complete() {
        //for all values of ReagentEffect and ReagentKind, there must be
        // a key in NAMES so that the lookup can work
        for kind in ReagentKind::iter() {
            if !(NAMES.contains_key(kind.to_string().to_lowercase().as_str())) {
                panic!("No entry found for {:?}", kind)
            };
        }

        for eff in ReagentEffect::iter() {
            if !(NAMES.contains_key(eff.to_string().to_lowercase().as_str())) {
                panic!("No entry found for {:?}", eff)
            }
        }
    }
}
