// Static lifetime

let name = "Sam";

let name: &'static str = "Sam";
// 'static is a special name (reserved name)
// it means this is the lifetime of the entire program.
// things that have 'static lifetime don't get allocated/deallocated
// they just always exist. Where are they in memory?
// They are in the binary itself.
// this are great for performance, they are not copied in the stack/heap
// and that's exactly why when you make a string in rust, you get the static
// lifetime by default like:

let name: &str = "Sam";

// rust just infer the lifetime, and say, ok you don't need to put the lifetime there
// i know that this is a string reference.

////////////////////////////////////////////////////////////
// another example is:
let hi: &'static str = "Hello world"!
// they are store in the application binary as very nice performance optimization
// to prevent copying them onto the heap