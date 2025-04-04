fn main() {
    println!(
        "color code for red : {}",
        color_to_number_without_match("red")
    );
    println!("color code for red : {}", color_to_number_with_match("red"));

    println!("Factorial of 5 is {}", recursive_factorial(5));
    println!("Factorial of 5 is {}", factorial(5));
    println!("Factorial of 4 is {}", factorial(4));
}

fn color_to_number_without_match(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_with_match(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn recursive_factorial(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        _ => num * recursive_factorial(num - 1),
    }
}

fn factorial(mut num: u32) -> u32 {
    let mut result: u32 = num;
    while num > 1 {
        result *= num - 1;
        num -= 1
    }

    return result;
}
