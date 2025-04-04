fn main() -> () {
    let week_days = 0..7;
    let letters = 'a'..='z';

    for day in week_days {
        print!("{day} ")
    }

    println!("\n");

    for char in letters {
        print!("{char}  ");
    }
    
    println!("\n");

    // range inclusive
    let a = 1..=31; // 0 -> 31

    // range exclusive
    let b = 1..31; // 0 -> 30
}
