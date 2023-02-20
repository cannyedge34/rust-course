let point: (i64, i64, i64) = (0,0,0);

let x = point.0;
let y = point.1;
let z = point.2;

// destructuring
let(x, y, z) = point;

// destructuring some of them
let(_, y, z) = point;

// destructuring only z
let(_, _, z) = point;

// how to mutate tuples?
let mut point: (i64, i64, i64) = (0,0,0);

let point.0 = 17;
let point.1 = 25;
let point.2 = 30;

//unit tuple
let unit: () = ();

// this is syntactic sugar of th next one
fn main() {
  // ...
};

fn main() -> () {
  // ...
};

// () it can be the equivalent of void() in other languages
let print_return_val: () = println!("Hi!");