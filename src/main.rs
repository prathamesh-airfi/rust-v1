fn main() {
    println!("{}", identity::<i32>(12));
    println!("{:?}", make_tuple::<&str, i32>("Hello", 24));
}

fn identity<T>(val: T) -> T {
    val
}

fn make_tuple<T, Q>(first: T, second: Q) -> (T, Q) {
    return (first, second);
}
