#[allow(unused_variables)]

fn main() {
    let pepperoni = "Pepperoni".to_string();
    let mushroom = "Mushroom".to_string();
    let sausages = "Sausages".to_string();
    let mut pizza_toppings = vec![pepperoni, mushroom, sausages];

    pizza_toppings[1] = "Mast Mushroom".into();
    pizza_toppings.push(String::from("Extra Cheese"));
}
