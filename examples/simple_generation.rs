use reagent_gen::*;
fn main() {
    println!("Generating Plant with Burning/Combusting properties...");
    println!("");

    let reagent = ReagentBuilder::new()
        .with_kind(ReagentKind::Plant)
        .with_effect(ReagentEffect::Burning)
        .build();

    println!("{:?}", reagent);
}
