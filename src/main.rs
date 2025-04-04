fn main() {
    let number = 5;
    println!("Is {number} even : {}", is_even(number));

    /* Basic If */
    if number > 5 {
        println!("Number is greater than 5");
    }

    /* IF Else Statement */
    if number % 2 == 0 {
        println!("Even number");
    } else {
        println!("Odd number");
    }

    /* IF Else IF Else ladder */
    if number > 0 {
        println!("Positive");
    } else if number < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    /* Using if as expression to return value */
    let result = if number % 2 == 0 { "Even" } else { "Odd" };

    println!("{number} is : {result}")
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 { true } else { false }
}
