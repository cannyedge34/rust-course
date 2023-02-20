let mut years: [i32; 3] = [1995, 1982, 1920]

let first_year = year[0];

// destructuring
let [_, second_year, third_year] = years;

years[2] = 1980;

// this will panic:
years[x] = 1980;

// arrays allow one type
for year in years.iter() {
  println!("Next year: {}", year + 1);
}

//in contrast tuples can have different types:
let years: (i32, i32, bool) = (1992, 1996, true);

// Sumary:
// Arrays can be iterated over
// Tuples and (Structs) cannot
// Array elements must all have the same type
// they are less flexible than tuples and structs