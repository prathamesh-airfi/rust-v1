#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Lowfat(10);
    if let Milk::Lowfat(percent) = my_beverage {
        println!("I got {percent} milk")
    }

    let oat_milk = Milk::NonDairy {
        kind: String::from("Oat"),
    };
    if let Milk::NonDairy { kind } = oat_milk {
        println!("I've got this non dairy milk of {kind}");
    } else {
        println!("I've got another milk variant")
    }
}
