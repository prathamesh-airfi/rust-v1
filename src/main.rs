fn main() -> () {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    dbg!(numbers, numbers);

    let tuple_example: (&str, i32, &str) = ("Prathamesh", 26, "Engineering");
    let (name, age, department) = tuple_example;
    // dbg!(tuple_example);
    dbg!(tuple_example.0);
    dbg!(tuple_example.1);
    dbg!(tuple_example.2);

    dbg!(name);
    dbg!(age);
    dbg!(department);
    
}
