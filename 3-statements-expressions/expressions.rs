// expressions evaluates to a value

cats > 1_000

cats > count_cats(cat_areas)

fn multiply_both(x: f64, y: f64) -> f64 {
  // return x * y; is a statement, finish with ; symbol
  x * y // this is a expression and we don't need the return
}

let message = if cats > 1 {
  "Multiple Cats"
} else if cats > 1_000 {
  "Too many cats"
} else {
  "Need more cats"
}; // important this semicolon