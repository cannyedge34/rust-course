
// takes ownership of years from caller of this method
// the caller says, hey i'm gonna give you years and you
// are responsible for cleaning up after finish the execution of the method
fn print_years(years: Vec<i32>) {
  for year in years.iter() {
    println!("Year: {}", year);
  }
} // dealloc(years)

fn main() {
  let years = vec![1990, 1995, 2000, 2010];

  // we pass years here
  print_years(years); // years got deallocated
  print_years(years); // use-after-free bug!, how rust prevent this¿
                      // rust compiler will raise an error: use of moved value: years...
                      // how to fix it?, we just need to return it, see next method
}

/////////////////////////////////////////////////////////////////////////////
fn print_years(years: Vec<i32>) -> Vec<i32> {
  for year in years.iter() {
    println!("Year: {}", year);
  }

  return years // we transfer the ownership back to the caller (main)
}

fn main() {
  let years = vec![1990, 1995, 2000, 2010];

  // this is not more sophisticated way to do this, but
  // it's valid to illustrate how rust transfers the ownership
  let years2 = print_years(years);
  let years3 = print_years(years2);
}

////////////////////////////////////////////////////////////////////////////
// we can use .clone(), we copy by value and not by reference.
// very similar to method clone in ruby
// the downside of clone() is the performance cost
fn print_years(years: Vec<i32>) {
  for year in years.iter() {
    println!("Year: {}", year);
  }
} // dealloc(years)

fn main() {
  let years = vec![1990, 1995, 2000, 2010];

  // it's a resource overall when you are starting with rust
  // and we don't know the correct techniques to avoid these bugs as
  // use after free
  print_years(years.clone());
  print_years(years2);
}