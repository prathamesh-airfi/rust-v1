/* Blueprint */
#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    /* Struct Instance */
    let mocha = make_coffee(String::from("Mocha"), 12.99, true);

    println!(
        "My {} this morning cost {}. It is {} that is was hot",
        mocha.name, mocha.price, mocha.is_hot
    );

    drink_coffee(&mocha);
    println!("{:?}", mocha);
    println!("{:#?}", mocha);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}