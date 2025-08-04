#[allow(unused_variables)]

fn main() {
    let mut pizza_diameters = vec![8, 10, 12, 14];
    /* Add element to last */
    pizza_diameters.push(16);
    pizza_diameters.push(18);

    /* Add element to specific index */
    pizza_diameters.insert(0, 6);
    println!("{:?}", pizza_diameters);

    /* Remove element from last index */
    let val = pizza_diameters.pop();

    /* Remove element from specific index */
    let val = pizza_diameters.remove(1);
}
