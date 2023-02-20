enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> Self {
        // we are destructuring the tuple here
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents: u64 = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::City => {
                let residents: u64 = 10_000;

                (
                    format!("a *city* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents: u64 = 1_000_000;

                (
                    format!("a *metropolis* of approximately {} residents", residents),
                    residents,
                )
            }
            _ => {
                let residents: u64 = 1_000;
                (
                    format!(
                        "an *unknown-size* city of approximately {} residents",
                        residents
                    ),
                    residents,
                )
            }
        };

        // we use here the attributes of destructuring + received argument
        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let small_city: City = City::new(CitySize::Metropolis, true);

    println!("This city is {}", small_city.description);

    if small_city.residents > 100_000 {
        println!("Woww!!")
    }
}
