#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suid: CardSuit,
}

fn main() {
    let first_card = CardSuit::Hearts;
    println!("{first_card:#?}")
}
