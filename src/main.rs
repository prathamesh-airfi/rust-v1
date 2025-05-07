fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");
    let string_slice = &action_hero[0..6]; // Slicing with length of bytes, 1 byte = 1 char, except special chars
    println!("{string_slice}");

    print_str_length(string_slice);
    print_str_length(&action_hero); // Deref Coercion

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_slice = &arr[..3];

    print_array_length(&arr);
    print_array_length(&arr_slice);

}

fn print_str_length(text: &str) {
    println!("String slice has length of {}", text.len());
}

fn print_array_length(arr: &[i32]) {
    println!("Array slice has size of {}", arr.len());
}