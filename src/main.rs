fn main() {
    /* Mutating but transferring the ownership */
    let burger = String::from("Burger");
    add_fries(burger);
    // burger.push_str("sf"); // Error

    /* Mutating without transferring the ownership */
    let mut pizza = String::from("Pizza");
    add_fries_2(&mut pizza);
    pizza.push_str("string"); // Not error

    let mut name = returning_function();
    name.push('c');
}

// Transfer of ownership
fn add_fries(mut meal: String) {
    meal.push_str(" And Fries");
    println!("{meal}")
}

// Without Transfer of ownership
fn add_fries_2(meal: &mut String) {
    meal.push_str(" And Fries");
    println!("{meal}");
}

fn returning_function() -> String {
    String::from("Returning Value")
}
