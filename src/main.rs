use std::f32::consts::PI;

fn main() {
    greet();
    greet_with_name("Prathamesh");
    println!("Square of 5 is {}", square(5));
    println!("PI : {}", get_pi());

    let positive = true;
    let number = match  positive {
        true => 10,
        false => 5
    };

    print!("{number}");

    generate_panic();
}

/* Function with no parameter and no return */
fn greet() -> () {
    println!("Hello There!");
}

/* Function with parameter and no return */
fn greet_with_name(name: &str) -> () {
    println!("Hello {name}");
}

/* Function with parameter and return value */
fn square(n: i32) -> i32 {
    n * n
}

/* Function with no parameter and return */
fn get_pi() -> f32{
    PI
}

/* Function who never return anything */
fn generate_panic() -> ! {
    panic!("Generating Panic")
}