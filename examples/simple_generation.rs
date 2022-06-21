use reagent_gen::*;
fn main() {
    println!("Generating Plant with Burning/Combusting properties...\n");

    let reagent = ReagentBuilder::new()
        .with_kind(ReagentKind::Plant)
        .with_effect(ReagentEffect::Burning)
        .build();

    println!("{:?}\n", reagent);
    println!("Generating 10 plants with random effects...\n");

    for _ in 0..10 {
        let reagent = ReagentBuilder::new()
            .with_kind(ReagentKind::Plant)
            .with_random_properties(4)
            .with_random_effects(2)
            .build();

        println!("{:?}", reagent);
    }
}
