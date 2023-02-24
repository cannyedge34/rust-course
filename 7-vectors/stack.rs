//stack and heap

fn increment_decrement(num: u8) {
  print_nums(num + 1, num - 1);
}

fn print_nums(x: u8, y: u8) {
  ...
}

increment_decrement(42);

// rust change the stack_lenght.
// 1 rust put things in the stack
// 2 function use these things
// 3 once the function exit, rust decrease the stack_length back down
// 4 once the program is finished, the stack is empty again!

// when function returns something

fn double_and_return(num: u8) -> u8 {
  return num * 2;
}

let x = double_and_return(30);


fn double_twice(num: u8) -> (u8, u8) {
  return (num * 2, num * 2);
}

let (x, y) = double_twice(30);

// what about this?
fn double_twice(num: u8) -> Vec<u8> {
  ...
}

let (x, y) = double_twice(30);

// to be returnable, size must be known at compile time
// we can return a struct because rust knows the size of one struct

struct VecMetadata {
  first_element_index: usize,
  length: usize,
  capacity: usize,
}

// stack_memory stack_bytes[stack_length - 1]

//The short answer is, they are go in the heap
