let nums = vec![1, 2, 3];

struct VecMetadata {
  first_element_index: usize
  length: usize,
  capacity: usize
}


// this slice:
let slice = &nums[0..2];

// slice metatada
struct SliceMetadata { let slice = &nums[0..2];
  first_element_index: usize
  length: usize,
}
// does:
// 1. first_element_index is the same
// 2. length is the same but it does not have a capacity because slices always refer
// to some other existing allocation, that's what & symbol comes from.
// 3. slices are actually reference types
// 4. it would be something like, i want to look at some "subslices" of an existing vec
// 5. Slices are never owned () they are not going to be deallocated. they are referencing some
// data that already allocated in the heap in some other data structure.
//6. again very important: slice doesn't own the elements, just references them, (somebody else is the owner)

// it works like this:
nums: Vec<u8>
slice: &[u8]

//string slices:
let str_slice: &str = string[3..7];

// if we want to get all the content of the vec as slice we can do:
nums.as_slice()
// or string
string.as_str()