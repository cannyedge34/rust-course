enum Color: { // <- Type
  Green, // <- Variant
  Yellow, // <- Variant
  Red, // <- Variant
}

let go = Color::Green;
let slow_down = Color::Yellow;
let stop = Color::Red;

let go: Color = Color::Green;
let slow_down: Color = Color::Yellow;
let stop: Color = Color::Red;

enum Color: {
  Green,
  Yellow,
  Red,
  Custom { red: u8, green: u8, yellow: u8 } //Struct syntax
}

let go: Color = Color::Green;
let stop: Color = Color::Red;
let purple: Color = Color::Custom {
  red: 100, green: 0, blue: 250
};

// we can do it as well with tuples:
enum Color: {
  Green,
  Yellow,
  Red,
  Custom(u8, u8, u8) //Struct syntax
}
let purple: Color = Color::Custom(100, 0, 250);

// pattern matching

let current_color = Color::Yellow;

// match "works" a little bit as a when/case in ruby or switch/case in javascript
// but there are also some differences:
//- no need break

match current_color {
  Color::Green => {
    println!("It was green!")
  }
  Color::Yellow => {
    println!("It was yellow!")
  }
  Color::Custom { red, green, blue } => {
    println!("{}, {}, {}", red, green, blue);
  }
  // tuple style
  // Color::Custom(red, green, blue) => {
  //   println!("{}, {}, {}", red, green, blue);
  // }
}

let color_str = match current_color {
  Color::Green => {
    println!("It was green!")
  }
  Color::Yellow => {
    println!("It was yellow!")
  }
  _ => {
    println!("It was something else!") // careful with this because the compiler will not raise any warning
  }
};