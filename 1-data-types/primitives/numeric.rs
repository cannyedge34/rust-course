fn main() {
  let mut x = 1;
  x = 3.2;

  // ^^^^^^^ expected integer, found `&str`
  x = "Hello";
}