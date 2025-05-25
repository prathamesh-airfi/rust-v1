fn main() {
    let mut my_arr = [10, 15, 20, 25, 30];
    let my_slice = &mut my_arr[2..4];
    println!("{:?}", my_slice);
    my_slice[0] = 21;
    println!("{:?}", my_slice);
}
