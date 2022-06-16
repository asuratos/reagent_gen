use once_cell::sync::Lazy;
use rand::seq::SliceRandom;

use std::collections::HashMap;


// TODO: this should eventually be read from a raw file, like a JSON or RON
static NAMES: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| {
    let mut names = HashMap::from([("plant", vec!["leaf"]), ("Burning", vec!["ember"])]);
    names
});

pub enum NameGenError {
    UnknownProperty,
    EmptyNameList
}

pub fn lookup_name_fragment(prop: &str) -> Result<String, NameGenError> {
    let lc_prop = prop.to_lowercase();

    if !NAMES.contains_key(lc_prop.as_str()) {
        return Err(NameGenError::UnknownProperty);
    }

    let names_list = NAMES.get(&lc_prop.as_str()).unwrap();
    let frag = names_list.choose(&mut rand::thread_rng());

    if frag.is_none() { return Err(NameGenError::EmptyNameList) }

    Ok(frag.unwrap().to_string())
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
