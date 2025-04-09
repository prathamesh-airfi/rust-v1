fn main() {
    let mut name = String::from("Prathamesh");
    let ref_1 = &mut name;
    let ref_2 = &name;
    println!("{ref_2}");
}
