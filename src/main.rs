#[allow(unused_variables)]

fn main() {
    let pizza_diameters: Vec<i32> = Vec::new();
    let pizza_diameters = Vec::<i32>::new();
    println!("{:?}", pizza_diameters);

    let pastas = Vec::<String>::new();
    println!("{:?}", pastas);

    // Direct value assignment
    let pizza_diameters = vec![8, 10, 12, 14];
    println!("{:?}", pizza_diameters);
}
