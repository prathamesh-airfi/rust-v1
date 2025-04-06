fn main() {
    let is_concert = true;
    let is_event = is_concert;
    // No ownership is moved
    println!("{} {}", is_concert, is_event);

    let sushi = "Salmon";
    let dinner = sushi;
    // No ownership is moved
    println!("{} {}", sushi, dinner);

    let sushi = String::from("Salmon");
    let dinner = sushi;
    // Ownership is moved
    // println!("{} {}", sushi, dinner);

    let mut sushi = String::from("Salmon");
    eat_meal(&mut sushi);
    println!("{sushi}")
}

fn eat_meal(meal: &mut String) {
    meal.clear();
}
