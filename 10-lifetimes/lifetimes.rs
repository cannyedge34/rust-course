let years: Vec<i64> = vec![
  1980, 1985, 1990, 1990, 2000, 2005, 2010
];

let eighties: [&i64] = &years[0..2];
let nineties: [&i64] = &years[2..4];

println!(
  "We have {} years in the nineties",
  nineties.len()
)

////////////////////////////////////////////////////////////////////////////////
struct Releases {
  years: &[i64],
  eighties: &[i64],
  nineties: &[i64]
}

fn jazz_releases(years: &[i64]) -> Releases {
  let eighties: &[i64] = &years[0..2];
  let nineties: &[i64] = &years[2..4];

  Releases {
    years,
    eighties,
    nineties
  }
}

let releases = {
  let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2000];
  jazz_releases(&all_years)
}; // dealloc(all_years)

let eighties = releases.eighties;
// use-after-free! why?

// 1. vec! of years is allocated in line 32
// 2. rust dealloc the memory in line 34 because unless it is being moved
//    to a different scope and ownership is being transferred, it's going to
//    deallocate as soon as it goes out of the scope.
// 3. all_years is deallocated in line 34.
// 4. We never do reference all_years ever again, which of course we could not
//    because it's out of scope of the brackets {}
// 5. What jazz_releases function does is: we give reference of the heap allocation
//    (in this case, slice of years) and inside of that function, is taking slices of all_years
// 6. All references are pointing to that same heap memory as was originally allocated
//    in the vec! call in line 32 and
// 7. it's stored in a "Release" struct which we then were reference later in line 36
// 8. We will get a rust compiler error
////////////////////////////////////////////////////////////////////////////////////////
// How rust knows that it should raise a compiler error using the concept of lifetimes?

/////////////////////VERY IMPORTANT THIS CONCEPT////////////
// lifetime of the all_years vec could be something like:
// the time between vec! is allocated
// and when it's going to get automatically deallocated
// lifetime begin when it's allocated and ends when it's deallocated
/////////////////////VERY IMPORTANT THIS CONCEPT////////////

let releases = {
  let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2000];
  jazz_releases(&all_years)
}; // dealloc(all_years)

let eighties = releases.eighties; // refers to all_years after its lifetime has ended
// another way of think about lifetimes is
// a way of tracking when an use-after-free bug might happen
// if we use something after lifetime is ended, that will be a use-after-free bug

////////////////////////////////////////////////////////////////////////////////////
// What can we do about this and how can we track it?

                // still within its lifetime
fn jazz_releases(years: &[i64]) -> Releases {}
                                   // the problem comes here (Releases depends on years)

let releases = {
  let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2000];
  jazz_releases(&all_years)
}; // dealloc(all_years)

// 1. When we call jazz_releases in line 80, we still have in the scope (highlighted area)
// 2. this thing has not been yet deallocated, so, there is no problem in this stage.
// 3. the problems seem to come somewhere after this stage. (after we pass the years param to the function)
let eighties = releases.eighties;