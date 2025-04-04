fn main() {
    let mut count = 0;

    /* Loop Statement */
    loop {
        count += 1;

        if count == 3 {
            continue;
        }

        println!("Hello There!");

        if count == 5 {
            break;
        }
    }

    /* While loop */
    let mut count = 0;
    while count < 5 {
        println!("Hello Prathamesh");
        count += 1;
    }

    println!("fibonacci till 10 : {}", calculate_fibonaccie(10))
}

fn calculate_fibonaccie(num: i32) -> i32 {
    match num {
        0 => 0,
        1 | 2 => 1,
        _ => calculate_fibonaccie(num - 1) + calculate_fibonaccie(num - 2),
    }
}
