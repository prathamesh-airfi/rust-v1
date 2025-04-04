fn main() {
    /* Simple Match */
    let evaluation = true;
    let value = match evaluation {
        true => 10,
        _ => 20,
    };
    println!("{value}");

    /* Match with multiple values and condition */
    let number = 12;
    match number {
        2 | 4 | 6 | 8 => println!("{number} is even"),
        val if val % 2 == 0 => println!("{number} is even"),
        _ => println!("{number} is odd"),
    }
}
