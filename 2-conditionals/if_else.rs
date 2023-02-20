if cats > 1 {
  println!("Multiple Cats");
} else {
  println!("Need more cats");
}

if cats > 1 {
  println!("Multiple Cats");
} else if cats > 1_000 {
  println!("Too many cats");
}
