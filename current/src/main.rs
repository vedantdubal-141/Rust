use std::path::Component::Prefix;

enum Flavor{
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_oz:f32,
}

fn print_drink(drink: Drink){
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Fruity => println!("Fruit"),

    }

    println!("oz:{:?}", drink.fluid_oz);
}
fn main() {
 let sweet = Drink{
     flavor: Flavor::Fruity,
     fluid_oz: 0.6,
 };

}