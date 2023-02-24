// The heap
// heap_memory heap_bytes[index_of_first_elem]

struct VecMetadata {
  first_element_index: usize,
  length: usize,
  capacity: usize,
}

let nums = Vec::with_capacity(5);

// with_capacity method: hey "bookkeeping system" find me u8 in an memory row that there are available
// there are not used by some other Vec, it returns the index of the first element where that's true
// vec! macro under the hood is actually calling with_capacity

// capacity: what determines how many total slots in memory we are taking up (capacity of 5)
// length: how many slots in memory we are currently taking (but we are currently taking 3 for example)

// in contrast the stack

// [u8, 3]
let mut array: [u8, 3] = [1, 2, 3]; // stack allocated because we know how bigger is the array in compile time
let mut vector: Vec<u8> = vec![1, 2, 3]; // heap allocated because there are dynamic length,
                                         // it does not have a size in compile time, that's the purpose of the heap
