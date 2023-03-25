// what we can do with the problem explained in lifetimes.rs file?

struct Releases<'y> {
  years: &'y [i64],
  eighties: &'y [i64],
  nineties: &'y [i64]
}
// a lifetime parameter named 'y

// we can add these lifetimes-annotations <'a>, &'a
fn jazz_releases<'a>(years: &'a [i64]) -> Releases<'a> {
  let eighties: &'a [i64] = &years[0..2];
  let nineties: &'a [i64] = &years[2..4];
  // all these things (&'a) are referencing the same heap memory
  // and because of that, they all have the same lifetime as that heap memory

  Releases {
    years,
    eighties,
    nineties
  }
}
// what this says is:

// 1. i'm defining a lifetimes parameter (we are giving a name to that lifetime)
// 2. we can give a name to these lifetimes and we can refer to it in our code.
// 3. chosen name is 'a (lifetimes always begin with an apostrophe)
// 4. We can use any name we want 'e, 'b, 'foo, 'bar...
// 5. once we choose the name we put between angle brackets <'a> and we can reference
//    inside the type annotation &'a

//////////////////////////////////////////////////////////////////////////////
// putting &'a and &'y in the code together, we know how to express inside code
// the actual relationship between these lifetimes

/////////////////////////////////////////////////////////////////////////////
// let's zoom back out and take a look our previous code
// where we had the potential use-after-free if we didn't get the compiler error

// remember we have the apostrophe 'a
fn jazz_releases<'a>(years: &'a [i64]) -> Releases<'a> {}

let releases = {
  let all_years: Vec<i64> = vec![1980, 1985, 1990, 1995, 2000, 2000];
  jazz_releases(&all_years)
}; // dealloc(all_years)

let eighties = releases.eighties;

//////////////////////////// VERY IMPORTANT ////////////////////////////
// 1. now the lifetime of all_years vec! is going to be added via function arguments
// 2. and propagated to the struct that the function returns
// 3. now when Releases come back, we can say, ahhh this thing (Releases)
   // has to have a lifetime <'a> and
   // that means this thing only exist inside of highlighted part
// 4. Outside of highlighted part, it's outside of the lifetime,
      // and it's going to be a bug use-after-free
// 5. Now, we do reference something from Releases,
      // knowing the Releases has <'a> lifetime which is now already ended
      // and we can see much more clearly that we are trying to use it
      // outside of highlighted area
// 6. The value of these lifetime annotations is helping us see this.
// 7. We don't necessarily need to use them in the code, depending on like the code
      // we are writing but in some cases they are required
// 8. for example when we use references in a struct:
      // struct Releases<'y> {
      //   years: &'y [i64],
      //   eighties: &'y [i64],
      //   nineties: &'y [i64]
      // }
      // it will not compile because rust would require you put explicit
      // lifetime annotation inside your struct that hold references
// 9. it's the same for enum (reference inside of it), same rules.
// 10. Other case where we use lifetimes is quite often using in documentation
       // it's important to include the concepts of lifetime on working on new pakages
/////////////////////////////// VERY IMPORTANT ////////////////////////////