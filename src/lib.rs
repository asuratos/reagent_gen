enum ReagentKind {
    Plant
}

enum ReagentProperty {
    Restorative,
    Explosive,
    Poisonous,
    Combustible
}

struct Reagent {
    kind: ReagentKind,
    name: String,
    properties: Vec<ReagentProperty>
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
