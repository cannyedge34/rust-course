let last_char = my_string.pop();
// let last_char: char = my_string.pop(); wrong!!

let last_char: Option<char> = my_string.pop();

// what is T?, T could be anything, it's a char in the previous case
enum Option<T> {
  None,
  Some(T),
}

// we don't need to use Option::Some?
// Option is special/built-in standard library and we don't need to namespaced
let email: Option<String> = Some(email_string);
let email: Option<String> = None;

// None is Option::None - the prefix is optional for Option

// Related type is Result
// Result is similar to option but instead of representing
// of the possible absence of the value, represent an operation that could fail
enum Result<O, E> {
  Ok(O),
  Err(E),
}

let success: Result<i64, String> = Ok(42);
let failure: Result<i64, String> = Err(str);
// the Result:: prefix is optional for Result

// they are the only enums that we can omit the prefix