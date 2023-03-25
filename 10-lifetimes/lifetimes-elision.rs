// in sometimes the compiler knows what the lifetimes must be and
// it does not require you to annotate them
// lifetime elision si something like type inference in the sense that
// it's a way to say, i could write up the type here but as the compiler knows
// this information, i'm not gonna bother you, it's gonna be fine.

fn jazz_releases<'a>(years: &'a [i64])

let releases Releases<'_> = {
  let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2000];
  jazz_releases(&all_years)
}

/////////////////////////////////////////////////////////////
//other example:

fn len<'a>(&'a self)
// is equal to:
fn len (&self)