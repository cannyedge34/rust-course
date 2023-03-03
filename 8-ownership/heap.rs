// struct VecMetadata { let nums = vec![1, 2, 3];
//   first_element_index: usize,
//   length: usize,
//   capacity: usize,
// }

// let nums = vec![1, 2, 3];
// how does the bookkeeping system know when those bytes no longer in use?
vec! calls alloc(3)
// "find 3 unused heap bytes in a row, and mark them as in-use"
// this is not a problem that we have in the stack because we know the length.
// remember this is the heap...

// this is where we have the concept "Manual memory management"
// allocating things on the heap is not a trivial task.

fn get_final_orders() -> i64 {
  // alloc, where rust says... these 4 bytes are not longer in use? (deallocate memory)
  // the question is more when...somebody needs to call deallocate function in a proper time
  // we can do it as in line 31. It works...
  let orders = vec![1, 2, 3, 4];

  // all of these things are happening in the stack, except line 18;
  let mut total_orders = 0;
  // dealloc(orders); "use-after-free bug"
  // the problem here is some other part of the program
  // that was running in parallel run into that memory and now, we are in troubles...

  // this for-in loop here is waiting for vec![1, 2, 3, 4] but they were deallocated in line 25
  // and other part of the program allocated these bytes with vec![79, 48, 300] and, when
  // we run the this for... bummm!!
  for order in orders.iter() {
    total_orders += orders;
  }

  // dealloc(orders); again, what if something else is using those bytes now?
  let final_orders = finish(total_orders);
  // dealloc(orders); free these bytes
  // dealloc here is not a problem
  return final_orders; // "double free bug",

  // all of these things are happening in the stack, except line 21;

  // manual memory management is error-prone;
  // other languages use garbage collector but the performance of the programing language is affected
  // rust makes sure we don't need deallocate memory and we don't need to deal with these bugs and..
  // we don't have the performance issues due to the garbage collector deallocating memory
}
