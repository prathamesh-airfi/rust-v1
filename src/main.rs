fn main() {
    let name = "Hello There!";
    let name_ref = &name;
    let name_ref_1 = &name_ref;
    let name_ref_2 = &name_ref_1;

    /* Manual Dereferencing */
    println!("{}", *name_ref);

    /* Automatic Dereferencing */
    println!("{}", name_ref_2);

    /* Actual Address Printing */
    println!("{:p}", name);
    println!("{:p}", name_ref);
    println!("{:p}", name_ref_1);
    println!("{:p}", name_ref_2);
}
