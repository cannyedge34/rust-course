fn main() {
  let city_name: &str = "Badajoz";

  println!("The city of {}:\n", city_name);

  print_population(adults: 1_256_526, kids: 120_293, buildings: 108_907)
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
  let population: u64 = adults + kids as u64;

  let buildings_per_person: f64 = buildings as f64 / population as f64;

  println!("population is {}", population);

  if buildings_per_person >= 1.0 {
    println!("every one can have their own building");
  } else {
    println!("Buildings must be shared");
  }
}