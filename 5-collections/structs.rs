struct Point {
  x: i64,
  y: i64,
  z: i64,
}

fn new_point(x: i64, y: i64, z: i64) -> Point {
  // we need to use the name of the struct
  Point { x: x, y: y, z: z }

  // syntax sugar
  Point { x, y, z }
}

let point = Point { x: 1, y: 2, z: 3 };
let x = point.x

// destructuring
let Point { x, y, z } = point;
let Point { x, y: _, z } = point;

//using explicit fields
let Point { x, z: .. } = point;
let Point { x, .. } = point;

// mutable struct
let mut point = Point { x: 1, y: 2, z: 3 }
point.x = 5