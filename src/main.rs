#[derive(Debug)]
struct Flight {
    origin: String,
    price: f64,
    passengers: u32,
    destination: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, passengers: u32, price: f64) -> Self {
        Self {
            price,
            passengers,
            origin: String::from(origin),
            destination: String::from(destination),
        }
    }
}

impl Flight {
    fn change_destination(&mut self, destination: &str) {
        self.destination = String::from(destination)
    }

    fn increase_price(&mut self) {
        self.price = self.price * 1.20;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination)
    }
}

fn main() {
    let mut flight = Flight::new("Surat", "Pune", 2, 12.90);
    flight.change_destination("Mumbai");
    flight.increase_price();
    flight.itinerary();
    println!("{flight:#?}");

    let mut goa_flight = Flight {
        origin: "Delhi".to_string(),
        destination: "Goa".to_string(),
        ..flight
    };

    println!("{:#?}", goa_flight)
}
