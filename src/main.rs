#[allow(unused_variables)]

fn main() {
    let pepperoni = "Pepperoni".to_string();
    let mushroom = "Mushroom".to_string();
    let sausages = "Sausages".to_string();
    let mut pizza_toppings = vec![pepperoni, mushroom, sausages];
    let sua = &pizza_toppings[2];
    let mush = &pizza_toppings[1];
    let delicious_toppings = &pizza_toppings;
    let pep = &mut pizza_toppings[0];
    // println!("{:#?}", pizza_toppings);
    // println!("{:#?}", mush);
    *pep = "PeP".to_string();
}
