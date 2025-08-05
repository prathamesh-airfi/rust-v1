#[allow(unused_variables)]

fn main() {
    let pizza_diameters = vec![8, 10, 12, 14];
    let pepperoni = "Pepperoni".to_string();
    let mushroom = "Mushroom".to_string();
    let sausages = "Sausages".to_string();
    let pizza_toppings = vec![pepperoni, mushroom, sausages];

    /* For datatype implementing Copy Trait */
    let second_index = pizza_diameters.get(1);
    let second_index = pizza_diameters[1];

    /* For datatype that doesn't implement Copy Trait */
    let second_index = &pizza_toppings[1];
    let second_index = pizza_toppings.get(1);

    /* Using slices */
    let slice = &pizza_diameters[0..2];
    println!("{:#?}", slice)
}
