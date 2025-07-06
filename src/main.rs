fn main() {
    let optional_value = Option::Some(12);
    let none_value = None::<i32>;

    let ok_value = Result::<bool, String>::Ok(true);
    let err_value = Result::<bool, String>::Err("dfdf".to_string());

    println!("{:#?}", optional_value);
    println!("{:#?}", none_value);
    println!("{:#?}", ok_value);
    println!("{:#?}", err_value);
}
