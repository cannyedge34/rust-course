// mutable vector
let mut years: Vec<i32> = vec![1990, 1995];

// mutable references to a vector (years vector)
let mutable_years: &mut Vec<i32> = &mut years;

let length = mutable_years.len();
// this will not compile as
// mutable references has additional restrictions that inmutable references do not have

mutable_years.clear();
// clear removes all elements from the Vec

//mutable reference to self
fn clear(&mut self) {
  // set self's length to 0
}

//inmutable reference to self
fn len(&self) -> usize

//////////////////////////////////////////////////////////////////////////
// no problem here
let years: Vec<i32> = vec![1990, 1995];

let years_ref1: &Vec<i32>            = &years;
                // this part is the
                // reference type     // this part is an inmutable borrow (the borrow part)

let years_ref2: &Vec<i32> = &years;
// no problem here
// we can can have multiple inmutable references at a time.

// Mutable reference restrictions
let years: Vec<i32> = vec![1990, 1995];
let years2: &mut Vec<i32> = &mut years;
// we can can only have one mutable references at a time.
// also, if we have in the scope the mutable reference,
// we are not allowed to have any inmutable references (line 32) at the same time.
// we should use to make it works:
let mut years: Vec<i32> = vec![1990, 1995];
let years2: &mut Vec<i32> = &mut years;

// we will have an error (cannot borrow years as mutable more than once at a time) if we try:
let years2: &mut Vec<i32> = &mut years;
// we will have an error -------first mutable borrow occurs here
let years3: &mut Vec<i32> = &mut years;
// we will have an error -------second mutable borrow occurs here

// we can not do this:
let mut years: Vec<i32> = vec![1990, 1995];
let years2: &Vec<i32> = &years;
let years3: &mut Vec<i32> = &mut years;

// we can not do this:
let mut years: Vec<i32> = vec![1990, 1995];
let years2: &Vec<i32> = &mut years;
let years3: &Vec<i32> = &years;
// we can only have one mutable reference active at a time or
// any number of immutable ones. They are exclusives, either one mutable
// or any number immutable but you can not mix them.
// the reason for this is concurrency (data races) due to multiple threads
// running on parallel, with shared memory, one of the threads writes to that
// piece of shared memory and at the same time there is another thread is trying to
// read from it and potentially it could happen is this race data condition. These issues
// are extremely complex to reproduce/debug.