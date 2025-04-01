fn main() {
    /*
        Methods 
            A method is a function that lives on a value.
            Its an action we can ask the value to execute.
    */

    let value: i32 = -15;
    println!("{}", value.abs());
    println!("Power {}", value.pow(2));
    println!("Power {}", value.pow(3));


    let empty_space = "        empty_space";
    println!("{}", empty_space.trim());
}
