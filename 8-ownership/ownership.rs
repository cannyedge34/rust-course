fn get_years() -> Vec<i32> {
  let years = vec![1995, 2000, 2005, 2010];
              // alloc (this scope or execution-context "owns" years)
  return years; // transfer ownership from this scope/execution-context to main
  // ("move" years to main's scope)
} // dealloc(years) because it went out of scope

// guaranteed no use-after-free

fn main() {
  let years = get_years();// take ownership
} // dealloc(years) because it went out of scope
                    // without being moved elsewhere