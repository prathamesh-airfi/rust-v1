// Hours, Minutes
struct ShortDuration(u32, u32);

// Years, Months
struct LongDuration(u32, u32);

struct EmptyStruct;

fn main() {
    let work_shift = ShortDuration(8, 30);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} Years {} Months", era.0, era.1);

    let empty = EmptyStruct;
}
