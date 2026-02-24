enum Flavour{
    Sparkling,
    Sweet,
    Fruity,
}

struct DrinkFlavour{
    flavour: Flavour,
    fluid_oz: f32,
}
fn print_both(a:DrinkFlavour){

    match a.flavour {
        Flavour::Sparkling => println!("Sparkling"),
        Flavour::Sweet => println!("Sweet"),
        Flavour::Fruity => println!("Fruity"),
    }

    println!("{:?}", a.fluid_oz);
}
fn main() {
    let sweet = DrinkFlavour {
        flavour: Flavour::Sparkling,
        fluid_oz: 12.0;
    }
}
