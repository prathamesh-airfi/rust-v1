#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    let Milk::Lowfat(percent) = my_beverage else {
        println!("I got other kind of milk");
        return;
    };

    println!("I've got {percent} percent of milk");
}
