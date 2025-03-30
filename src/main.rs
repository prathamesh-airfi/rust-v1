fn main() {
    let apples = 10;
    let oranges = 20;
    let fruits = apples + oranges;
    println!("This year, my garden has {} apples.", apples); // Interpolated Arguments
    println!("This year, my graden has total {fruits} fruits"); // Interpolated Arguments
    println!(
        "This year, my garden has {0} apples, {1} oragnes and total {2} fruits",
        apples, oranges, fruits
    ); // Positional Arguments
}
