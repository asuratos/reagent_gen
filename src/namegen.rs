use std::collections::HashMap;

use once_cell::sync::Lazy;
use rand::seq::SliceRandom;

use crate::ReagentBuilder;

// TODO: this should eventually be read from a raw file, like a JSON or RON
static NAMES: Lazy<HashMap<&str, Vec<&str>>> =
    Lazy::new(|| HashMap::from([("plant", vec!["leaf"]), ("burning", vec!["ember"])]));

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

    let names_list = NAMES.get(&lc_prop.as_str()).unwrap();
    let frag = names_list.choose(&mut rand::thread_rng());

    if frag.is_none() {
        return Err(NameGenError::EmptyNameList);
    }

    Ok(frag.unwrap().to_string())
}

pub fn new_name(builder: &ReagentBuilder) -> Result<String, NameGenError> {
    // TODO

    // get &str version of kind for lookup
    // lookup name fragment for kind

    let kind: String;
    if let Some(k) = &builder.kind {
        kind = k.to_string();
    } else {
        return Err(NameGenError::UninitializedKind);
    }

    // get &str of one of the properties for lookup
    // lookup name fragment for property
    let prop = "ember";

    // use template
    let template = "{{prop}}{{kind}}";

    Ok(template
        .replace("{{prop}}", prop)
        .replace("{{kind}}", &kind))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_property() {
        assert!(lookup_name_fragment("tag doesn't exist").is_err());
    }

    #[test]
    fn name_fragment_args_are_case_insensitive() {
        assert!(lookup_name_fragment("Plant").is_ok());
        assert!(lookup_name_fragment("plant").is_ok());
    }
}
