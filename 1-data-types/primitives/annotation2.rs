fn main() {
  let answer = multiply_both(1.2, 2.3);

  println!("x time y is {}", answer);
}

// we need to use -> symbol when function return something
// we use kind of data (f64) that the method return
fn multiply_both(x: f64, y: f64) -> f64 {
  return x * y;
}