#[derive(Debug)]
enum CheeseSteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom: CheeseSteak<String> = CheeseSteak::<String>::Topping(String::from("Mushroom"));
    let plain_cheese_steak = CheeseSteak::<String>::Plain;

    println!("{:?}", mushroom);
    println!("{:?}", plain_cheese_steak);
}
