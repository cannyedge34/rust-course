enum Color {

}

impl Color {
  fn new(r: u8, g: u8, b: u8) -> Color {...}
  fn rgb(color: Color) -> (u8, u8, u8) {...}
}

let red = Color::new(250, 0, 0);
let purple = Color::new(100, 0, 250);
let (r, g, b) = Color::rgb(purple);

//--------------------------------------------------------------
impl Color {
  fn new(r: u8, g: u8, b: u8) -> Self {...} // Self, special value, only used inside impl, in this case is Color
  fn rgb(self) -> (u8, u8, u8) {...} // self, special value, only used inside impl, in this case is color: Color
}

let purple: Color = Color::new(100, 0, 250);
let (r, g, b) = Color::rgb(purple); // syntatic sugar
let (r, g, b) purple.rgb(); // syntatic sugar