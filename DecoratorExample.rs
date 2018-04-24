#![allow(unused)]

trait Beverage {
    fn description(&self) -> String;
    fn cost(&self) -> f64;
    
    fn print(&self) {
        println!("{} ... $ {}", self.description(), self.cost());
    }
}

struct HouseBlend;
impl Beverage for HouseBlend {
    fn description(&self) -> String { "House Blend Coffe".into() }
    fn cost(&self) -> f64 { 0.89 }
}

struct Espresso;
impl Beverage for Espresso {
    fn description(&self) -> String { "Espresso".into() }
    fn cost(&self) -> f64 { 1.99 }
}

struct Milk<B: Beverage>(B);
impl<B: Beverage> Beverage for Milk<B> {
    fn description(&self) -> String { self.0.description() + ", Milk" }
    fn cost(&self) -> f64 { self.0.cost() + 0.10 }
}

struct Soy<B: Beverage>(B);
impl<B: Beverage> Beverage for Soy<B> {
    fn description(&self) -> String { self.0.description() + ", Soy" }
    fn cost(&self) -> f64 { self.0.cost() + 0.15 }
}

fn main() {
    let b1 = Milk(Espresso);
    b1.print();
    let b2 = Soy(Soy(HouseBlend));
    b2.print();
}
