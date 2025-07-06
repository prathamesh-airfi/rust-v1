#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        match self.has_mice_infestation {
            true => None::<Food>,
            false if self.reservations > 12 => Some(Food {
                name: "Strip Steak".to_string(),
            }),
            false => Some(Food {
                name: "Uni Sashimi".to_string(),
            }),
        }
    }

    fn deliver_burger(&self, addr: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        }
        match addr.is_empty() {
            true => Err(String::from("No delivery address specified")),
            false => Ok(self.chef_special().unwrap()),
        }
    }
}

fn main() {
    let restaurant_1 = Restaurant {
        has_mice_infestation: true,
        reservations: 11,
    };

    println!("{:?}", restaurant_1.chef_special());
    println!("{:?}", restaurant_1.deliver_burger("123 Elm Street"));

    let restaurant_2 = Restaurant {
        has_mice_infestation: false,
        reservations: 15,
    };
    println!("{:?}", restaurant_2.chef_special());
    println!("{:?}", restaurant_2.deliver_burger(""));
    println!("{:?}", restaurant_2.deliver_burger("123 Elm Street"));
}
