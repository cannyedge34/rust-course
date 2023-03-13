fn print_years(years: Vec<i32>) {
  for year in years.iter() {
    println!("Year: {}", year);
  }
} // dealloc(years)

fn main() {
  let years = vec![1990, 1995, 2000, 2010];

  // we can't "turn off the borrow checker in Rust" https://steveklabnik.com/writing/you-can-t-turn-off-the-borrow-checker-in-rust
  print_years(years); // compile error value moved here
  print_years(years); // value used here after move
}

// Solution
                      // this is a references type
                      // this is not a Vec<i32> rather it is a reference to a Vec<i32>
fn print_years(years: &Vec<i32>) {
  for year in years.iter() {
    println!("Year: {}", year);
  }
}

fn main() {
  let years = vec![1990, 1995, 2000, 2010];

  // temporarily give print_years access to years
  // (&years - "i want to borrow years", i'm not going to transfer the ownership)
  // i'm just letting you borrow it
  print_years(&years);
  print_years(&years);
}

// this is a reference to a self/caller of this function
fn len(&self) -> usize