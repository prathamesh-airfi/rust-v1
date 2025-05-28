struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

/* Associated Methods */
impl TaylorSwiftSong {
    fn new(title: &str, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title: title.to_string(),
            release_year,
            duration_secs,
        }
    }
}

/* Instance Methods */
impl TaylorSwiftSong {
    /* Immutable struct value (self parameter takes ownership) (self: Self) */
    /* Mutable struct value  (self parameter takes ownership, has permission to mutate) (self: mut Self) */
    /* Immutable reference to struct instance (No Ownership Moved) (self: &Self) */
    /* mutable reference to struct instance (No Ownership Moved) (self: &mut Self) */
    fn display_song_info(&self) {
        println!("\n");
        println!("Title: {}", self.title);
        println!("Year Since Release : {}", self.year_since_release());
        println!("Duration: {} seconds", self.duration_secs);
        println!("---------------------------------------------")
    }

    fn double_length(&mut self) {
        self.duration_secs *= 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn year_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}

fn main() {
    let mut blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    let all_too_well = TaylorSwiftSong::new("All Too Well", 2025, 234);
    blank_space.display_song_info();
    blank_space.double_length();
    blank_space.display_song_info();
    all_too_well.display_song_info();

    println!(
        "is blank_space is longer than all_too_well : {}",
        blank_space.is_longer_than(&all_too_well)
    );
}
