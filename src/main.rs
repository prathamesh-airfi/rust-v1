enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => println!("Running the laundry with cold temperature"),
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with temperature of {temperature}")
            }

            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with delicate cycle for {fabric_type}")
            }
        }
    }
}

fn main() {
    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 14 }.wash_laundry();
    LaundryCycle::Delicate("Cotton".to_string()).wash_laundry();
}
