#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}
impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&mut self) {
        self.captain = self.captain.to_uppercase();
    }
}

fn main() {
    let mut gold_chest = TreasureChest {
        captain: "Firebeard".to_string(),
        treasure: "Gold",
    };

    gold_chest.capital_captain();
    println!("{:#?}", gold_chest);

    let mut silver_chest = TreasureChest {
        captain: "Bloodsail".to_string(),
        treasure: "Silver".to_string(),
    };

    silver_chest.capital_captain();
    silver_chest.clean_treasure();
    println!("{:#?}", silver_chest);

    let mut special_chest = TreasureChest {
        captain: "Bootyplunder".to_string(),
        treasure: ["Gold", "Silver", "Platinum"],
    };

    special_chest.capital_captain();
    special_chest.amount_of_treasure();
    println!("{:#?}", special_chest);
}
